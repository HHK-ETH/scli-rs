use crate::{
    graphql::queries::periodVolumeQuery::{period_volume_query, PeriodVolumeQuery},
    helpers::{
        block::block_timestamp::{query_block_timestamp, BlockQueryError},
        token_list::query_token_list,
    },
    network, subgraph,
};
use graphql_client::{GraphQLQuery, Response};
use std::{collections::HashMap, thread};

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
