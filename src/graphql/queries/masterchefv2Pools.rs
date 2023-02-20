#![allow(clippy::all, warnings)]
pub struct MasterChefv2Pools;
type BigInt = String;
type Bytes = String;
pub mod masterchefv2_pools {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "masterChefv2Pools";
    pub const QUERY : & str = "query masterChefv2Pools {\n  masterChefs {\n    id\n    totalAllocPoint\n  }\n  pools(\n    first: 1000\n    orderBy: allocPoint\n    orderDirection: desc\n    where: {allocPoint_gt: 0}\n  ) {\n    id\n    pair\n    allocPoint\n  }\n}" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type BigInt = super::BigInt;
    type Bytes = super::Bytes;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "masterChefs")]
        pub master_chefs: Vec<MasterChefv2PoolsMasterChefs>,
        pub pools: Vec<MasterChefv2PoolsPools>,
    }
    #[derive(Deserialize, Debug)]
    pub struct MasterChefv2PoolsMasterChefs {
        pub id: ID,
        #[serde(rename = "totalAllocPoint")]
        pub total_alloc_point: BigInt,
    }
    #[derive(Deserialize, Debug)]
    pub struct MasterChefv2PoolsPools {
        pub id: ID,
        pub pair: Bytes,
        #[serde(rename = "allocPoint")]
        pub alloc_point: BigInt,
    }
}
impl graphql_client::GraphQLQuery for MasterChefv2Pools {
    type Variables = masterchefv2_pools::Variables;
    type ResponseData = masterchefv2_pools::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: masterchefv2_pools::QUERY,
            operation_name: masterchefv2_pools::OPERATION_NAME,
        }
    }
}
