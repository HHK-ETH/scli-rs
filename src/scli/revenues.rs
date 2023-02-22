use std::{cmp::Ordering, collections::HashMap};

use clap::{Arg, ArgMatches, Command};

use crate::{
    helpers::{
        farm::pools_with_sushi::{query_pools_with_sushi, Minichef},
        volume::period_volume::{query_period_volume, Pair},
    },
    network::MINICHEF_SUBGRAPH,
};

pub fn command() -> Command {
    let network_arg = Arg::new("network")
        .help("Network to query. Ex: ethereum.")
        .short('n')
        .long("network");
    let days_arg = Arg::new("days")
        .help("Days in the past to query. Ex: 7 (for weekly revenue)")
        .short('d')
        .long("days")
        .default_value("1");

    let revenues = Command::new("revenues")
        .about("Get revenues for all chains by comparing fees with sushi spent.")
        .arg(network_arg)
        .arg(days_arg);

    revenues
}

fn parse_days(days: &String) -> u32 {
    let days = match days.parse::<u32>() {
        Ok(days) => days,
        Err(error) => {
            eprintln!(
                "Error while parsing days param, make sure to enter a valid number. {}",
                error
            );
            panic!()
        }
    };
    if days > 30 || days == 0 {
        eprintln!("Please enter days between 1 and 30.");
        panic!()
    }
    days
}

#[derive(Debug, Clone)]
struct PairRevenues {
    name: String,
    volume: f64,
    fees: f64,
    spent: f64,
}

impl PairRevenues {
    pub fn new(pair: &Pair, sushi_amount: f64, sushi_price: f64) -> PairRevenues {
        PairRevenues {
            name: pair.name.clone(),
            volume: pair.volume_usd,
            fees: pair.fees_usd,
            spent: sushi_amount * sushi_price,
        }
    }
}

#[derive(Debug)]
struct ChainRevenues {
    chain: String,
    total_volume: f64,
    total_fees: f64,
    total_spent: f64,
    best: Vec<PairRevenues>,
    worst: Vec<PairRevenues>,
}

fn compute_revenues(
    chain: String,
    days: u32,
    volumes: HashMap<String, Pair>,
    minichef: Option<Minichef>,
) -> ChainRevenues {
    let sushi_price = 1.5;

    let mut total_volume = 0.0;
    let mut total_fees = 0.0;
    let mut total_spent = 0.0;

    let mut pairRevenues: Vec<PairRevenues> = vec![];
    if minichef.is_none() {
        for pair in volumes.values() {
            total_volume += pair.volume_usd;
            total_fees += pair.fees_usd;

            pairRevenues.push(PairRevenues::new(pair, 0.0, sushi_price))
        }
    } else {
        let minichef = minichef.unwrap();
        total_spent = minichef.sushi_per_day * sushi_price * days as f64;
        for pair in volumes.values() {
            total_volume += pair.volume_usd;
            total_fees += pair.fees_usd;

            let mut sushi_amount = 0.0;
            let pool = minichef.pools.contains_key(&pair.id);
            if pool {
                sushi_amount = minichef.pools.get(&pair.id).unwrap().sushi_per_day * days as f64;
            }
            pairRevenues.push(PairRevenues::new(pair, sushi_amount, sushi_price))
        }
    }

    pairRevenues.sort_by(|a, b| {
        let earned_a = a.fees - a.spent;
        let earned_b = b.fees - b.spent;

        if earned_a > earned_b {
            return Ordering::Less;
        };
        return Ordering::Greater;
    });

    ChainRevenues {
        chain,
        total_volume,
        total_fees,
        total_spent,
        best: pairRevenues[0..3].to_vec(),
        worst: pairRevenues[(pairRevenues.len() - 3)..pairRevenues.len()].to_vec(),
    }
}

pub fn execute(params: &ArgMatches) -> () {
    let network = params.get_one::<String>("network");
    let days = parse_days(params.get_one::<String>("days").unwrap()); //default to 1

    if network.is_some() {
        let chain = network.unwrap();
        let volume = match query_period_volume(chain.clone(), days) {
            Ok(volume) => volume,
            Err(error) => {
                eprintln!("Error while querying volume: {:#?}", error);
                return;
            }
        };

        let mut minichef = None;
        if chain.contains("ethereum") || MINICHEF_SUBGRAPH.contains_key(chain) {
            minichef = match query_pools_with_sushi(chain.clone()) {
                Ok(minichef) => Some(minichef),
                Err(error) => {
                    eprintln!("Error while querying farms: {:#?}", error);
                    return;
                }
            };
        }

        let revenues = compute_revenues(chain.clone(), days, volume, minichef);
        println!("{:#?}", revenues);
    } else {
    }
}
