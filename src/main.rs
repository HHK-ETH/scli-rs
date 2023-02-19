mod graphql;
mod helpers;
mod network;
mod subgraph;

fn main() {
    let weekly_volume =
        helpers::volume::period_volume::query_period_volume(vec!["arbitrum".to_string()], 7);

    println!("{:#?}", weekly_volume);

    let arbitrum = weekly_volume.get("arbitrum").unwrap();

    let mut total_volume = 0.0;
    let mut total_fees = 0.0;
    for pair in arbitrum.values() {
        total_volume += pair.volume_usd;
        total_fees += pair.fees_usd;
    }

    println!("Arbitrum volume: {} usd", total_volume.round());
    println!("Arbitrum fees: {} usd", total_fees.round());
}
