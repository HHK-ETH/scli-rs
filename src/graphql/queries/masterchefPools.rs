#![allow(clippy::all, warnings)]
pub struct MasterchefPools;
type BigInt = String;
type Bytes = String;
pub mod masterchef_pools {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "masterchefPools";
    pub const QUERY : & str = "query masterchefPools{\n  masterChefs {\n    id\n    sushiPerBlock\n    totalAllocPoint\n  }\n  pools(\n    first: 1000\n    orderBy: allocPoint\n    orderDirection: desc\n    where: {allocPoint_gt: 0}\n  ) {\n    id\n    pair\n    allocPoint\n  }\n}" ;
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
        pub master_chefs: Vec<MasterchefPoolsMasterChefs>,
        pub pools: Vec<MasterchefPoolsPools>,
    }
    #[derive(Deserialize, Debug)]
    pub struct MasterchefPoolsMasterChefs {
        pub id: ID,
        #[serde(rename = "sushiPerBlock")]
        pub sushi_per_block: BigInt,
        #[serde(rename = "totalAllocPoint")]
        pub total_alloc_point: BigInt,
    }
    #[derive(Deserialize, Debug)]
    pub struct MasterchefPoolsPools {
        pub id: ID,
        pub pair: Bytes,
        #[serde(rename = "allocPoint")]
        pub alloc_point: BigInt,
    }
}
impl graphql_client::GraphQLQuery for MasterchefPools {
    type Variables = masterchef_pools::Variables;
    type ResponseData = masterchef_pools::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: masterchef_pools::QUERY,
            operation_name: masterchef_pools::OPERATION_NAME,
        }
    }
}
