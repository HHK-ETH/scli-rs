#![allow(clippy::all, warnings)]

use ethers::types::U256;
pub struct MinichefPools;
type BigInt = String;
type Bytes = String;
pub mod minichef_pools {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "farms";
    pub const QUERY : & str = "query farms {\n  miniChefs {\n    id\n    sushiPerSecond\n    totalAllocPoint\n  }\n  pools(first: 1000, where: {allocPoint_gt: 0}) {\n    id\n    pair\n    allocPoint\n  }\n}" ;
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
        #[serde(rename = "miniChefs")]
        pub mini_chefs: Vec<FarmsMiniChefs>,
        pub pools: Vec<FarmsPools>,
    }
    #[derive(Deserialize, Debug)]
    pub struct FarmsMiniChefs {
        pub id: ID,
        #[serde(rename = "sushiPerSecond")]
        pub sushi_per_second: BigInt,
        #[serde(rename = "totalAllocPoint")]
        pub total_alloc_point: BigInt,
    }
    #[derive(Deserialize, Debug)]
    pub struct FarmsPools {
        pub id: ID,
        pub pair: Bytes,
        #[serde(rename = "allocPoint")]
        pub alloc_point: BigInt,
    }
}
impl graphql_client::GraphQLQuery for MinichefPools {
    type Variables = minichef_pools::Variables;
    type ResponseData = minichef_pools::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: minichef_pools::QUERY,
            operation_name: minichef_pools::OPERATION_NAME,
        }
    }
}
