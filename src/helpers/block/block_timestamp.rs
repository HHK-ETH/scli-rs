use crate::{
    graphql::queries::blockByTimestamp::{block_by_timestamp, BlockByTimestamp},
    network, subgraph,
};
use graphql_client::{GraphQLQuery, Response};
use std::{
    num::ParseIntError,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
pub enum BlockQueryError {
    NoSubgraphUrl(String),
    RequestError(String),
    EmptyResponse,
    ParsingError(ParseIntError),
}

pub fn query_block_timestamp(chain: &str, days: u32) -> Result<i64, BlockQueryError> {
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
