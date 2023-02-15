use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex, MutexGuard},
    thread::{self},
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
    query_path = "src/graphql/queries/exchange-v2.graphql",
    response_derives = "Debug, Clone"
)]
pub struct DailyVolumeQuery;

pub fn query_daily_volume(
) -> Result<HashMap<String, daily_volume_query::ResponseData>, Box<dyn Error>> {
    let legacy_subgrahps = network::LEGACY_SUBGRAPH.entries();

    let time: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() //unlikely to panic
        .as_secs() as i64;

    let mut handles = vec![];
    let chain_data: Arc<Mutex<HashMap<String, daily_volume_query::ResponseData>>> =
        Arc::new(Mutex::new(HashMap::new()));

    for (chain, subgraph) in legacy_subgrahps {
        let chain_data_copy = Arc::clone(&chain_data);
        let handle = thread::spawn(move || {
            let request_body = DailyVolumeQuery::build_query(daily_volume_query::Variables {
                date_start: time - 3600 * 48, //48 hours from now
                date_end: time - 3600 * 24,   //24 hours from now
            });

            let res: Response<daily_volume_query::ResponseData> =
                subgraph::query_subgraph(subgraph, &request_body).unwrap();

            let mut chain_data: MutexGuard<HashMap<String, daily_volume_query::ResponseData>> =
                chain_data_copy.lock().unwrap();
            chain_data.insert(chain.to_string(), res.data.unwrap());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }

    let result = chain_data.lock().unwrap();
    Ok(result.clone())
}
