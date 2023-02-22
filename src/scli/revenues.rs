use std::{cmp::Ordering, collections::HashMap, error::Error};

use clap::{Arg, ArgMatches, Command};
use cli_table::{print_stdout, Cell, CellStruct, Table};
use serde::Deserialize;

use crate::{
    helpers::{
        farm::pools_with_sushi::{
            query_multichain_pools_with_sushi, query_pools_with_sushi, Minichef,
        },
        volume::period_volume::{query_period_volume, query_period_volume_multichain, Pair},
    },
    network::{LEGACY_SUBGRAPH, MINICHEF_SUBGRAPH},
};

#[derive(Deserialize)]
struct Prices {
    coins: HashMap<String, Price>,
}

#[derive(Deserialize)]
struct Price {
    decimals: u32,
    symbol: String,
    price: f64,
    timestamp: u32,
    confidence: f64,
}

fn query_sushi_price() -> Result<f64, Box<dyn Error>> {
    let url = "https://coins.llama.fi/prices/current/ethereum:0x6b3595068778dd592e39a122f4f5a5cf09c90fe2?searchWidth=4h";
    let client = reqwest::blocking::Client::new();
    let res = client.get(url).send()?;
    let prices: Prices = res.json()?;
    let price = prices
        .coins
        .get("ethereum:0x6b3595068778dd592e39a122f4f5a5cf09c90fe2")
        .unwrap()
        .price;
    Ok(price)
}

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
    sushi_price: f64,
) -> ChainRevenues {
    let mut total_volume = 0.0;
    let mut total_fees = 0.0;
    let mut total_spent = 0.0;

    let mut pair_revenues: Vec<PairRevenues> = vec![];
    if minichef.is_none() {
        for pair in volumes.values() {
            total_volume += pair.volume_usd;
            total_fees += pair.fees_usd;

            pair_revenues.push(PairRevenues::new(pair, 0.0, sushi_price))
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
            pair_revenues.push(PairRevenues::new(pair, sushi_amount, sushi_price))
        }
    }

    pair_revenues.sort_by(|a, b| {
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
        best: if pair_revenues.len() > 3 {
            pair_revenues[0..3].to_vec()
        } else {
            [].into()
        },
        worst: if pair_revenues.len() > 3 {
            pair_revenues[(pair_revenues.len() - 3)..pair_revenues.len()].to_vec()
        } else {
            [].into()
        },
    }
}

pub fn execute(params: &ArgMatches) -> () {
    let sushi_price = match query_sushi_price() {
        Ok(price) => price,
        Err(error) => {
            eprintln!("Error while querying sushi price: {:#?}", error);
            return;
        }
    };
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

        let revenues = compute_revenues(chain.clone(), days, volume, minichef, sushi_price);
        println!("{:#?}", revenues);
    } else {
        let chains: Vec<String> = LEGACY_SUBGRAPH
            .keys()
            .map(|chain| chain.to_string())
            .collect();
        let volumes = query_period_volume_multichain(chains.clone(), days);

        let mut minichef_chains: Vec<String> = MINICHEF_SUBGRAPH
            .keys()
            .map(|chain| chain.to_string())
            .collect();
        minichef_chains.push("ethereum".to_string());
        let mut minichefs = query_multichain_pools_with_sushi(minichef_chains);

        let mut revenues: Vec<ChainRevenues> = vec![];
        for volume in volumes {
            let chain = volume.0;
            let volume = volume.1;
            let minichef = minichefs.remove(&chain);
            revenues.push(compute_revenues(chain, days, volume, minichef, sushi_price));
        }

        revenues.sort_by(|a, b| {
            if a.total_fees > b.total_fees {
                return Ordering::Less;
            }
            return Ordering::Greater;
        });
        let revenues_table: Vec<Vec<CellStruct>> = revenues
            .iter()
            .map(|revenue| {
                vec![
                    revenue.chain.as_str().cell(),
                    format!("{} $", revenue.total_volume.round()).cell(),
                    format!("{} $", revenue.total_fees.round()).cell(),
                    format!("{} $", revenue.total_spent.round()).cell(),
                    format!("{} $", (revenue.total_fees - revenue.total_spent).round()).cell(),
                ]
            })
            .collect();
        let revenues_table = revenues_table.table().title(vec![
            "Chain".cell(),
            "Volume".cell(),
            "Fees".cell(),
            "Spent".cell(),
            "Revenue".cell(),
        ]);

        print_stdout(revenues_table);
    }
}
