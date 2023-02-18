mod daily_volume;
mod graphql;
mod network;
mod subgraph;

fn main() {
    let weekly_volume = daily_volume::query_period_volume(7).unwrap();

    println!("{:#?}", weekly_volume);
}
