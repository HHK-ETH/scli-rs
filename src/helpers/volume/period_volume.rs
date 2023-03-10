use crate::{
    graphql::queries::periodVolumeQuery::{period_volume_query, PeriodVolumeQuery},
    helpers::{
        block::block_timestamp::{query_block_timestamp, BlockQueryError},
        token_list::query_token_list,
    },
    network::LEGACY_SUBGRAPH,
    subgraph,
};
use graphql_client::{GraphQLQuery, Response};
use std::{collections::HashMap, thread};

#[derive(Debug)]
pub struct Pair {
    pub id: String,
    pub name: String,
    pub volume_usd: f64,
    pub fees_usd: f64,
}

impl Pair {
    pub fn from(id: String, name: String, volume_usd: String, fees_usd: String) -> Option<Pair> {
        let volume_usd: f64 = match volume_usd.parse() {
            Ok(volume) => volume,
            Err(_) => return None, //don't return pair if can't compute volume
        };
        let fees_usd: f64 = match fees_usd.parse() {
            Ok(fees) => fees,
            Err(_) => return None, //don't return pair if can't compute fees
        };
        Some(Pair {
            id,
            name,
            volume_usd,
            fees_usd,
        })
    }
}

fn parse_volume(volume: period_volume_query::ResponseData) -> HashMap<String, Pair> {
    let mut pairs: HashMap<String, Pair> = HashMap::new();

    for new_pair_state in volume.new_pairs_state {
        match Pair::from(
            new_pair_state.id,
            new_pair_state.name,
            new_pair_state.volume_usd,
            new_pair_state.fees_usd,
        ) {
            Some(pair) => pairs.insert(pair.id.clone(), pair),
            None => break, //don't add pair if None
        };
    }

    for old_pair_state in volume.old_pairs_state {
        let mut pair = match pairs.get_mut(&old_pair_state.id) {
            Some(pair) => pair,
            None => break, //cancel if cannot find pair
        };
        let old_volume_usd: f64 = match old_pair_state.volume_usd.parse() {
            Ok(volume) => volume,
            Err(_) => {
                //remove pair if can't compute old volume
                pairs.remove(&old_pair_state.id);
                break;
            }
        };
        let old_fees_usd: f64 = match old_pair_state.fees_usd.parse() {
            Ok(fees) => fees,
            Err(_) => {
                //remove pair if can't compute old fees
                pairs.remove(&old_pair_state.id);
                break;
            }
        };
        pair.volume_usd -= old_volume_usd;
        pair.fees_usd -= old_fees_usd;
    }

    for newly_created_pair in volume.newly_created_pairs {
        match Pair::from(
            newly_created_pair.id,
            newly_created_pair.name,
            newly_created_pair.volume_usd,
            newly_created_pair.fees_usd,
        ) {
            Some(pair) => pairs.insert(pair.id.clone(), pair),
            None => break, //don't add pair if None
        };
    }

    pairs
}

#[derive(Debug)]
pub enum PeriodVolumeQueryError {
    UnknownChain(String),
    BlockQueryError(String, BlockQueryError),
    RequestError(String, String),
    EmptyResponse(String),
}

pub fn query_period_volume(
    chain: String,
    days: u32,
) -> Result<HashMap<String, Pair>, PeriodVolumeQueryError> {
    let subgraph = match LEGACY_SUBGRAPH.get(chain.as_str()) {
        Some(subgraph) => subgraph,
        None => return Err(PeriodVolumeQueryError::UnknownChain(chain)),
    };

    let block = match query_block_timestamp(chain.as_str(), days) {
        Ok(block) => block,
        Err(error) => return Err(PeriodVolumeQueryError::BlockQueryError(chain, error)),
    };

    let token_list: Option<Vec<String>> = match query_token_list(chain.as_str()) {
        Ok(token_list) => Some(token_list),
        Err(error) => {
            eprintln!("Error while querying token list: {:#?}", error);
            None
        }
    };

    let volume_request_body =
        PeriodVolumeQuery::build_query(period_volume_query::Variables { token_list, block });

    let res: Response<period_volume_query::ResponseData> =
        match subgraph::query_subgraph(subgraph, &volume_request_body) {
            Ok(res) => res,
            Err(error) => {
                return Err(PeriodVolumeQueryError::RequestError(
                    chain,
                    error.to_string(),
                ))
            }
        };

    match res.data {
        Some(data) => Ok(parse_volume(data)),
        None => Err(PeriodVolumeQueryError::EmptyResponse(chain)),
    }
}

pub fn query_period_volume_multichain(
    chains: Vec<String>,
    days: u32,
) -> HashMap<String, HashMap<String, Pair>> {
    let mut handles: Vec<
        thread::JoinHandle<
            Result<(String, HashMap<std::string::String, Pair>), PeriodVolumeQueryError>,
        >,
    > = vec![];
    for chain in chains {
        let handle = thread::spawn(move || match query_period_volume(chain.clone(), days) {
            Ok(volume) => Ok((chain, volume)),
            Err(error) => Err(error),
        });
        handles.push(handle);
    }

    let mut chain_data: HashMap<String, HashMap<String, Pair>> = HashMap::new();
    for handle in handles {
        match handle.join().unwrap() {
            Ok(volume) => chain_data.insert(volume.0, volume.1),
            Err(error) => {
                eprintln!("Error while querying volume: {:#?}", error);
                continue;
            }
        };
    }

    chain_data
}
