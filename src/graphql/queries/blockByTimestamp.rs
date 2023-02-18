#![allow(clippy::all, warnings)]
pub struct BlockByTimestamp;
type BigInt = String;
pub mod block_by_timestamp {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "BlockByTimestamp";
    pub const QUERY : & str = "query BlockByTimestamp($timestamp: BigInt!) {\n    blocks(\n    first: 1\n    orderBy: number\n    orderDirection: desc\n    where: {timestamp_lt: $timestamp}\n  ) {\n    id\n    number\n  }\n}" ;
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
    #[derive(Serialize)]
    pub struct Variables {
        pub timestamp: BigInt,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub blocks: Vec<BlockByTimestampBlocks>,
    }
    #[derive(Deserialize, Debug)]
    pub struct BlockByTimestampBlocks {
        pub id: ID,
        pub number: BigInt,
    }
}
impl graphql_client::GraphQLQuery for BlockByTimestamp {
    type Variables = block_by_timestamp::Variables;
    type ResponseData = block_by_timestamp::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: block_by_timestamp::QUERY,
            operation_name: block_by_timestamp::OPERATION_NAME,
        }
    }
}
