use crate::{network, subgraph};
use graphql_client::{GraphQLQuery, Response};
use std::{
    collections::HashMap,
    error::Error,
    num::ParseIntError,
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::graphql::queries::blockByTimestamp::{block_by_timestamp, BlockByTimestamp};
use crate::graphql::queries::periodVolumeQuery::{period_volume_query, PeriodVolumeQuery};

fn query_token_list(chain: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let chain_id = match network::NETWORKS.get(chain) {
        Some(network) => network.chain_id,
        None => 0,
    };

    let client = reqwest::blocking::Client::new();
    let result = client
        .get(format!("https://helper.sushibackup.com/tokens/{chain_id}"))
        .send()?;
    let token_list: Vec<String> = result.json()?;
    let token_list = token_list
        .iter()
        .map(|token| token.to_lowercase())
        .collect();
    Ok(token_list)
}

#[derive(Debug)]
pub enum BlockQueryError {
    NoSubgraphUrl(String),
    RequestError(String),
    EmptyResponse,
    ParsingError(ParseIntError),
}

fn query_block_timestamp(chain: &str, days: u32) -> Result<i64, BlockQueryError> {
    let time: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() //unlikely to panic
        .as_secs() as u64;

    let timestamp = time - u64::from(86_400 * days);

    let block_request_body =
        BlockByTimestamp::build_query(block_by_timestamp::Variables { timestamp });

    let block_subgraph_url = match network::BLOCK_SUBGRAPH.get(chain) {
        Some(url) => url,
        None => return Err(BlockQueryError::NoSubgraphUrl(chain.to_string())),
    };

    let block_res: Response<block_by_timestamp::ResponseData> =
        match subgraph::query_subgraph(block_subgraph_url, &block_request_body) {
            Ok(res) => res,
            Err(error) => return Err(BlockQueryError::RequestError(error.to_string())),
        };

    let block_string = match block_res.data {
        Some(block) => block.blocks[0].number.clone(),
        None => return Err(BlockQueryError::EmptyResponse),
    };

    match block_string.parse() {
        Ok(block) => return Ok(block),
        Err(error) => return Err(BlockQueryError::ParsingError(error)),
    };
}

#[derive(Debug)]
pub enum PeriodVolumeQueryError {
    BlockQueryError(String, BlockQueryError),
    RequestError(String, String),
    EmptyResponse(String),
}

pub fn query_period_volume(days: u32) -> HashMap<String, period_volume_query::ResponseData> {
    let legacy_subgrahps = network::LEGACY_SUBGRAPH.entries();

    let mut handles: Vec<
        thread::JoinHandle<
            Result<(String, period_volume_query::ResponseData), PeriodVolumeQueryError>,
        >,
    > = vec![];
    for (chain, subgraph) in legacy_subgrahps {
        let handle = thread::spawn(move || {
            let block = match query_block_timestamp(chain, days) {
                Ok(block) => block,
                Err(error) => {
                    return Err(PeriodVolumeQueryError::BlockQueryError(
                        chain.to_string(),
                        error,
                    ))
                }
            };

            let token_list: Option<Vec<String>> = match query_token_list(chain) {
                Ok(token_list) => Some(token_list),
                Err(error) => {
                    eprintln!("Error while querying token list: {:#?}", error);
                    None
                }
            };
            let volume_request_body =
                PeriodVolumeQuery::build_query(period_volume_query::Variables {
                    token_list,
                    block,
                });

            let res: Response<period_volume_query::ResponseData> =
                match subgraph::query_subgraph(subgraph, &volume_request_body) {
                    Ok(res) => res,
                    Err(error) => {
                        return Err(PeriodVolumeQueryError::RequestError(
                            chain.to_string(),
                            error.to_string(),
                        ))
                    }
                };

            match res.data {
                Some(data) => return Ok((chain.to_string(), data)),
                None => return Err(PeriodVolumeQueryError::EmptyResponse(chain.to_string())),
            };
        });
        handles.push(handle);
    }

    let mut chain_data: HashMap<String, period_volume_query::ResponseData> = HashMap::new();
    for handle in handles {
        let data = match handle.join().unwrap() {
            Ok(data) => Some(data),
            Err(error) => {
                eprintln!("Error while querying volume: {:#?}", error);
                None
            }
        };
        if !data.is_none() {
            let data = data.unwrap();
            chain_data.insert(data.0, data.1);
        }
    }

    chain_data
}
