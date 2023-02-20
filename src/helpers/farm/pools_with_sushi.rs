use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
};

use ethers::{types::U256, utils::format_units};
use graphql_client::{GraphQLQuery, Response};

use crate::{
    graphql::queries::{
        masterchefPools::{masterchef_pools, MasterchefPools},
        masterchefv2Pools::{masterchefv2_pools, MasterChefv2Pools},
        minichefPools::{
            minichef_pools::{self, FarmsPools},
            MinichefPools,
        },
    },
    network::{MASTERCHEFV2_SUBGRAPH, MASTERCHEFV2_TOKEN, MASTERCHEF_SUBGRAPH, MINICHEF_SUBGRAPH},
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
            id: data.pair,
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
        let sushi_per_second =
            match U256::from_dec_str(data.mini_chefs[0].sushi_per_second.as_str()) {
                Ok(sushi_per_second) => sushi_per_second,
                Err(_) => return None,
            };
        let sushi_per_day: f64 = match format_units(sushi_per_second, 18) {
            Ok(sushi_per_second) => sushi_per_second.parse::<f64>().unwrap() * 86_400.0,
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

        Some(minichef)
    }

    pub fn from_mainnet(
        msv1: masterchef_pools::ResponseData,
        msv2: masterchefv2_pools::ResponseData,
    ) -> Option<Minichef> {
        let msv1_alloc_point: u32 =
            match U256::from_dec_str(msv1.master_chefs[0].total_alloc_point.as_str()) {
                Ok(total_alloc_point) => total_alloc_point.as_u32(),
                Err(_) => return None,
            };

        let msv2_alloc_point: u32 =
            match U256::from_dec_str(msv2.master_chefs[0].total_alloc_point.as_str()) {
                Ok(total_alloc_point) => total_alloc_point.as_u32(),
                Err(_) => return None,
            };

        let sushi_per_block =
            match U256::from_dec_str(msv1.master_chefs[0].sushi_per_block.as_str()) {
                Ok(sushi_per_second) => sushi_per_second,
                Err(_) => return None,
            };
        let sushi_per_day: f64 = match format_units(sushi_per_block, 18) {
            Ok(sushi_per_second) => sushi_per_second.parse::<f64>().unwrap() / 12.0 * 86_400.0, //12 secs in a block
            Err(_) => return None,
        };

        let mut minichef = Minichef {
            id: msv1.master_chefs[0].id.clone(),
            sushi_per_day,
            total_alloc_point: msv1_alloc_point,
            pools: HashMap::new(),
        };

        let mut msv2_pool: Option<Pool> = None;
        for pool_data in msv1.pools {
            let pool = match Pool::from(
                FarmsPools {
                    id: pool_data.id,
                    pair: pool_data.pair,
                    alloc_point: pool_data.alloc_point,
                },
                msv1_alloc_point,
                sushi_per_day,
            ) {
                Some(pool) => pool,
                None => break,
            };
            if pool.id.contains(MASTERCHEFV2_TOKEN) {
                msv2_pool = Some(pool); //save msv2 and don't add it to pools
            } else {
                minichef.pools.insert(pool.id.clone(), pool);
            }
        }

        if msv2_pool.is_some() {
            let msv2_pool = msv2_pool.unwrap();
            for pool_data_v2 in msv2.pools {
                let mut pool_v2 = match Pool::from(
                    FarmsPools {
                        id: pool_data_v2.id,
                        pair: pool_data_v2.pair,
                        alloc_point: pool_data_v2.alloc_point,
                    },
                    msv2_alloc_point,
                    msv2_pool.sushi_per_day, //sushi given to msv2 pool by msv1
                ) {
                    Some(pool) => pool,
                    None => break,
                };
                //get the new alloc_point converted for msv1 /!\ rounded /!\
                pool_v2.alloc_point = (pool_v2.alloc_point as f32 / msv2_alloc_point as f32
                    * msv2_pool.alloc_point as f32) as u32;
                minichef.pools.insert(pool_v2.id.clone(), pool_v2);
            }
        }

        Some(minichef)
    }
}

#[derive(Debug)]
pub enum PoolsWithSushiQueryError {
    UnknownChain(String),
    RequestError(String, String),
    EmptyResponse(String),
    ParsingMinichef(String),
}

fn query_mainnet_pools_with_sushi() -> Result<Minichef, PoolsWithSushiQueryError> {
    let chain = "ethereum".to_string();
    let msv1_query = MasterchefPools::build_query(masterchef_pools::Variables);

    let msv1: Response<masterchef_pools::ResponseData> =
        match subgraph::query_subgraph(MASTERCHEF_SUBGRAPH, &msv1_query) {
            Ok(msv1) => msv1,
            Err(error) => {
                return Err(PoolsWithSushiQueryError::RequestError(
                    chain,
                    error.to_string(),
                ))
            }
        };

    let msv1 = match msv1.data {
        Some(data) => data,
        None => return Err(PoolsWithSushiQueryError::EmptyResponse(chain)),
    };

    let msv2_query = MasterChefv2Pools::build_query(masterchefv2_pools::Variables);

    let msv2: Response<masterchefv2_pools::ResponseData> =
        match subgraph::query_subgraph(MASTERCHEFV2_SUBGRAPH, &msv2_query) {
            Ok(msv2) => msv2,
            Err(error) => {
                return Err(PoolsWithSushiQueryError::RequestError(
                    chain,
                    error.to_string(),
                ))
            }
        };

    let msv2 = match msv2.data {
        Some(data) => data,
        None => return Err(PoolsWithSushiQueryError::EmptyResponse(chain)),
    };

    match Minichef::from_mainnet(msv1, msv2) {
        Some(minichef) => return Ok(minichef),
        None => return Err(PoolsWithSushiQueryError::ParsingMinichef(chain)),
    };
}

pub fn query_pools_with_sushi(chain: String) -> Result<Minichef, PoolsWithSushiQueryError> {
    if chain.contains("ethereum") {
        return query_mainnet_pools_with_sushi();
    }
    let subgraph = match MINICHEF_SUBGRAPH.get(&chain) {
        Some(subgraph) => subgraph,
        None => return Err(PoolsWithSushiQueryError::UnknownChain(chain)),
    };

    let minichef_pools_query = MinichefPools::build_query(minichef_pools::Variables);

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
        Some(minichef) => return Ok(minichef),
        None => return Err(PoolsWithSushiQueryError::ParsingMinichef(chain)),
    };
}

pub fn query_multichain_pools_with_sushi(chains: Vec<String>) -> HashMap<String, Minichef> {
    let mut handles: Vec<JoinHandle<Result<(String, Minichef), PoolsWithSushiQueryError>>> = vec![];
    for chain in chains {
        let handle = thread::spawn(move || {
            match query_pools_with_sushi(chain.clone()) {
                Ok(minichef) => return Ok((chain, minichef)),
                Err(error) => return Err(error),
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
