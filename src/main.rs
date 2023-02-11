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
pub struct MyQuery;

fn main() {
    let request_body = MyQuery::build_query(my_query::Variables);

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("https://api.thegraph.com/subgraphs/name/sushi-v2/sushiswap-ethereum")
        .json(&request_body)
        .send();
    let response_body: Response<my_query::ResponseData> = res.unwrap().json().unwrap();
    println!("{:#?}", response_body);
}
