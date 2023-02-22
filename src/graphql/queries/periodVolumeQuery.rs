#![allow(clippy::all, warnings)]
pub struct PeriodVolumeQuery;
type BigDecimal = String;
pub mod period_volume_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PeriodVolumeQuery";
    pub const QUERY : & str = "query PeriodVolumeQuery($tokenList: [String!], $block: Int!) {\n  oldPairsState: pairs(\n    first: 1000\n    orderBy: liquidityUSD\n    orderDirection: desc\n    where: { token0_in: $tokenList, token1_in: $tokenList, volumeUSD_gt: 0 }\n    block: {number: $block}\n  ) {\n    id\n    name\n    volumeUSD\n    feesUSD\n  }\n  newPairsState: pairs(\n    first: 1000\n    orderBy: liquidityUSD\n    orderDirection: desc\n    where: { token0_in: $tokenList, token1_in: $tokenList, createdAtBlock_lte: $block, volumeUSD_gt: 0 }\n  ) {\n    id\n    name\n    volumeUSD\n    feesUSD\n  }\n  newlyCreatedPairs: pairs(\n    first: 100\n    orderBy: volumeUSD\n    orderDirection: desc\n    where: { token0_in: $tokenList, token1_in: $tokenList, createdAtBlock_gt: $block, volumeUSD_gt: 0 }\n  ) {\n    id\n    name\n    volumeUSD\n    feesUSD\n  }\n}" ;
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
    type BigDecimal = super::BigDecimal;
    #[derive(Serialize, Debug)]
    pub struct Variables {
        #[serde(rename = "tokenList")]
        pub token_list: Option<Vec<String>>,
        pub block: Int,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        #[serde(rename = "oldPairsState")]
        pub old_pairs_state: Vec<PeriodVolumeQueryOldPairsState>,
        #[serde(rename = "newPairsState")]
        pub new_pairs_state: Vec<PeriodVolumeQueryNewPairsState>,
        #[serde(rename = "newlyCreatedPairs")]
        pub newly_created_pairs: Vec<PeriodVolumeQueryNewlyCreatedPairs>,
    }
    #[derive(Deserialize, Debug)]
    pub struct PeriodVolumeQueryOldPairsState {
        pub id: ID,
        pub name: String,
        #[serde(rename = "volumeUSD")]
        pub volume_usd: BigDecimal,
        #[serde(rename = "feesUSD")]
        pub fees_usd: BigDecimal,
    }
    #[derive(Deserialize, Debug)]
    pub struct PeriodVolumeQueryNewPairsState {
        pub id: ID,
        pub name: String,
        #[serde(rename = "volumeUSD")]
        pub volume_usd: BigDecimal,
        #[serde(rename = "feesUSD")]
        pub fees_usd: BigDecimal,
    }
    #[derive(Deserialize, Debug)]
    pub struct PeriodVolumeQueryNewlyCreatedPairs {
        pub id: ID,
        pub name: String,
        #[serde(rename = "volumeUSD")]
        pub volume_usd: BigDecimal,
        #[serde(rename = "feesUSD")]
        pub fees_usd: BigDecimal,
    }
}
impl graphql_client::GraphQLQuery for PeriodVolumeQuery {
    type Variables = period_volume_query::Variables;
    type ResponseData = period_volume_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: period_volume_query::QUERY,
            operation_name: period_volume_query::OPERATION_NAME,
        }
    }
}
