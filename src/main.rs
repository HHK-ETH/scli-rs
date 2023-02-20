use helpers::farm::pools_with_sushi::query_multichain_pools_with_sushi;

mod graphql;
mod helpers;
mod network;
mod subgraph;

fn main() {
    let farms = query_multichain_pools_with_sushi(vec!["ethereum".to_string()]);

    println!("{:#?}", farms);
}
