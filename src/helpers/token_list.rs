use crate::network;
use std::error::Error;

pub fn query_token_list(chain: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let chain_id = match network::NETWORKS.get(chain) {
        Some(network) => network.chain_id,
        None => 0,
    };

    let client = reqwest::blocking::Client::new();
    let result = client
        .get(format!("https://helper.sushibackup.com/tokens/{chain_id}"))
        .send()?;
    let token_list: Vec<String> = result.json()?;
    let token_list = token_list
        .iter()
        .map(|token| token.to_lowercase())
        .collect();
    Ok(token_list)
}
