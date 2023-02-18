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
    let client = reqwest::blocking::Client::new();
    let result = client
        .get(format!("https://helper.sushibackup.com/tokens/{chain}"))
        .send()?;
    let token_list: Vec<String> = result.json()?;
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

            let token_list: Option<Vec<String>> = Some(vec![
                "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_lowercase(),
                "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_lowercase(),
                "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619".to_lowercase(),
                "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174".to_lowercase(),
            ]);
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
