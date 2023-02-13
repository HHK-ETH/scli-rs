use std::error::Error;

use serde::{de::DeserializeOwned, Serialize};

pub fn query_subgraph<T, U>(
    url: &str,
    request_body: &T,
) -> Result<graphql_client::Response<U>, Box<dyn Error>>
where
    T: Serialize + ?Sized,
    U: std::fmt::Debug + DeserializeOwned,
{
    let client = reqwest::blocking::Client::new();
    let res: reqwest::blocking::Response = client.post(url).json(request_body).send()?;
    let response_body: graphql_client::Response<U> = res.json()?;
    return Ok(response_body);
}
