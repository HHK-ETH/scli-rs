use std::{
    collections::HashMap,
    error::Error,
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{network, subgraph};
use graphql_client::{GraphQLQuery, Response};

type BigDecimal = String;

// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schemas/exchange-v2.json",
    query_path = "src/graphql/queries/pairsByTvl.graphql",
    response_derives = "Debug, Clone"
)]
pub struct PairsByTvl;

pub fn query_daily_volume() -> Result<HashMap<String, pairs_by_tvl::ResponseData>, Box<dyn Error>> {
    let legacy_subgrahps = network::LEGACY_SUBGRAPH.entries();

    /*let time: i64 = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap() //unlikely to panic
    .as_secs() as i64;*/

    let mut handles = vec![];
    for (chain, subgraph) in legacy_subgrahps {
        let handle = thread::spawn(move || {
            /*let request_body = PairsByTvl::build_query(pairs_by_tvl::Variables {
                timestamp_low: time - 3600 * 48, //48 hours from now
                date_end: time - 3600 * 24,      //24 hours from now
            });*/

            let token_list: Option<Vec<String>> = Some(vec!["".to_string()]);
            let request_body = PairsByTvl::build_query(pairs_by_tvl::Variables { token_list });

            let res: Response<pairs_by_tvl::ResponseData> =
                subgraph::query_subgraph(subgraph, &request_body).unwrap();

            println!("{:#?}", res);

            return (chain.to_string(), res.data.unwrap());
        });
        handles.push(handle);
    }

    let mut chain_data: HashMap<String, pairs_by_tvl::ResponseData> = HashMap::new();
    for handle in handles {
        let data = handle.join().unwrap();
        chain_data.insert(data.0, data.1);
    }

    Ok(chain_data)
}
