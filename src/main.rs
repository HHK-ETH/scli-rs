mod daily_volume;
mod network;
mod subgraph;

fn main() {
    let daily_volume = daily_volume::query_daily_volume().unwrap();

    println!("{:#?}", daily_volume);
}
