use crate::{network, subgraph};
use graphql_client::{GraphQLQuery, Response};
use std::{
    collections::HashMap,
    error::Error,
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::graphql::queries::blockByTimestamp::{block_by_timestamp, BlockByTimestamp};
use crate::graphql::queries::periodVolumeQuery::{period_volume_query, PeriodVolumeQuery};

pub fn query_period_volume(
    days: u32,
) -> Result<HashMap<String, period_volume_query::ResponseData>, Box<dyn Error>> {
    let legacy_subgrahps = network::LEGACY_SUBGRAPH.entries();

    let time: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() //unlikely to panic
        .as_secs() as u64;

    let mut handles = vec![];
    for (chain, subgraph) in legacy_subgrahps {
        let handle = thread::spawn(move || {
            let timestamp = time - u64::from(86_400 * days);

            let block_request_body =
                BlockByTimestamp::build_query(block_by_timestamp::Variables { timestamp });

            let block_subgraph_url = network::BLOCK_SUBGRAPH.get(chain).unwrap();
            let block_res: Response<block_by_timestamp::ResponseData> =
                subgraph::query_subgraph(block_subgraph_url, &block_request_body).unwrap();

            let block = block_res.data.unwrap();
            let block = block.blocks[0].number.clone().parse().unwrap();

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
                subgraph::query_subgraph(subgraph, &volume_request_body).unwrap();

            return (chain.to_string(), res.data.unwrap());
        });
        handles.push(handle);
    }

    let mut chain_data: HashMap<String, period_volume_query::ResponseData> = HashMap::new();
    for handle in handles {
        let data = handle.join().unwrap();
        chain_data.insert(data.0, data.1);
    }

    Ok(chain_data)
}
