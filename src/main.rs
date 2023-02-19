mod graphql;
mod helpers;
mod network;
mod subgraph;

fn main() {
    let weekly_volume = helpers::volume::period_volume::query_period_volume(7);

    println!("{:#?}", weekly_volume);
}
