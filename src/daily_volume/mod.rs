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
            let block_request_body = BlockByTimestamp::build_query(block_by_timestamp::Variables {
                timestamp: timestamp.to_string(),
            });
            let block_subgraph_url = network::BLOCK_SUBGRAPH.get(chain).unwrap();
            let block_res: Response<block_by_timestamp::ResponseData> =
                subgraph::query_subgraph(block_subgraph_url, &block_request_body).unwrap();
            let block = block_res.data.unwrap();
            let block = block.blocks[0].number.clone();

            let token_list: Option<Vec<String>> = Some(vec!["".to_string()]);
            let volume_request_body =
                PeriodVolumeQuery::build_query(period_volume_query::Variables {
                    token_list,
                    block,
                });

            let res: Response<period_volume_query::ResponseData> =
                subgraph::query_subgraph(subgraph, &volume_request_body).unwrap();

            println!("{:#?}", res);

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
