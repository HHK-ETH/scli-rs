use std::{
    collections::HashMap,
    str::FromStr,
    thread::{self, JoinHandle},
};

use ethers::{types::U256, utils::format_units};
use graphql_client::{GraphQLQuery, Response};

use crate::{
    graphql::queries::minichefPools::{minichef_pools, MinichefPools},
    network::MINICHEF_SUBGRAPH,
    subgraph,
};

#[derive(Debug)]
pub struct Pool {
    pub id: String,
    pub alloc_point: u32,
    pub sushi_per_day: f64,
}

impl Pool {
    pub fn from(
        data: minichef_pools::FarmsPools,
        total_alloc_point: u32,
        total_sushi_per_day: f64,
    ) -> Option<Pool> {
        let alloc_point: u32 = match U256::from_dec_str(data.alloc_point.as_str()) {
            Ok(alloc_point) => alloc_point.as_u32(),
            Err(_) => return None,
        };

        let sushi_per_day = alloc_point as f64 / total_alloc_point as f64 * total_sushi_per_day;

        return Some(Pool {
            id: data.id,
            alloc_point,
            sushi_per_day,
        });
    }
}

#[derive(Debug)]
pub struct Minichef {
    pub id: String,
    pub sushi_per_day: f64,
    pub total_alloc_point: u32,
    pub pools: HashMap<String, Pool>,
}

impl Minichef {
    pub fn from(data: minichef_pools::ResponseData) -> Option<Minichef> {
        println!("{}", data.mini_chefs[0].sushi_per_second);
        let sushi_per_second =
            match U256::from_dec_str(data.mini_chefs[0].sushi_per_second.as_str()) {
                Ok(sushi_per_second) => sushi_per_second,
                Err(_) => return None,
            };
        let sushi_per_day: f64 = match format_units(sushi_per_second, 18) {
            Ok(sushi_per_second) => {
                println!("{}", sushi_per_second);
                sushi_per_second.parse::<f64>().unwrap() * 86_400.0
            }
            Err(_) => return None,
        };

        let total_alloc_point: u32 =
            match U256::from_dec_str(data.mini_chefs[0].total_alloc_point.as_str()) {
                Ok(total_alloc_point) => total_alloc_point.as_u32(),
                Err(_) => return None,
            };

        let mut minichef = Minichef {
            id: data.mini_chefs[0].id.clone(),
            sushi_per_day,
            total_alloc_point,
            pools: HashMap::new(),
        };

        for pool_data in data.pools {
            let pool = Pool::from(pool_data, total_alloc_point, sushi_per_day);
            match pool {
                Some(pool) => minichef.pools.insert(pool.id.clone(), pool),
                None => break,
            };
        }

        return Some(minichef);
    }
}

#[derive(Debug)]
enum PoolsWithSushiQueryError {
    UnknownChain(String),
    RequestError(String, String),
    EmptyResponse(String),
    ParsingMinichef(String),
}

pub fn query_pools_with_sushi(chains: Vec<String>) -> HashMap<String, Minichef> {
    let mut handles: Vec<JoinHandle<Result<(String, Minichef), PoolsWithSushiQueryError>>> = vec![];
    for chain in chains {
        let handle = thread::spawn(move || {
            //todo add something for ethereum
            let subgraph = match MINICHEF_SUBGRAPH.get(&chain) {
                Some(subgraph) => subgraph,
                None => return Err(PoolsWithSushiQueryError::UnknownChain(chain)),
            };

            let minichef_pools_query = MinichefPools::build_query(minichef_pools::Variables {});

            let minichef: Response<minichef_pools::ResponseData> =
                match subgraph::query_subgraph(subgraph, &minichef_pools_query) {
                    Ok(data) => data,
                    Err(error) => {
                        return Err(PoolsWithSushiQueryError::RequestError(
                            chain,
                            error.to_string(),
                        ))
                    }
                };

            let minichef = match minichef.data {
                Some(data) => Minichef::from(data),
                None => return Err(PoolsWithSushiQueryError::EmptyResponse(chain)),
            };
            match minichef {
                Some(minichef) => return Ok((chain, minichef)),
                None => return Err(PoolsWithSushiQueryError::ParsingMinichef(chain)),
            };
        });

        handles.push(handle);
    }

    let mut result = HashMap::new();
    for handle in handles {
        let minichef = handle.join().unwrap();
        match minichef {
            Ok(res) => result.insert(res.0, res.1),
            Err(error) => {
                eprintln!("Error while querying minichef: {:#?}", error);
                break;
            }
        };
    }
    return result;
}
