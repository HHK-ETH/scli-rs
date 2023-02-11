use std::time::{SystemTime, UNIX_EPOCH};

use graphql_client::{GraphQLQuery, Response};
use reqwest;

type BigDecimal = String;

// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schemas/exchange-v2.json",
    query_path = "src/graphql/queries/exchange-v2.graphql",
    response_derives = "Debug"
)]
pub struct DailyVolume;

fn main() {
    let time: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() //unlikely to panic
        .as_secs() as i64;

    let request_body = DailyVolume::build_query(daily_volume::Variables {
        date_start: time - 3600 * 24, //24 hours from now
        date_end: time,
    });

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://api.thegraph.com/subgraphs/name/sushi-v2/sushiswap-ethereum")
        .json(&request_body)
        .send();
    let response_body: Response<daily_volume::ResponseData> = res.unwrap().json().unwrap();
    println!("{:#?}", response_body);
}
