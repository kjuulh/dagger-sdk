use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use base64::engine::general_purpose;
use base64::Engine;
use gql_client::ClientConfig;
use serde::Deserialize;

use crate::connect_params::ConnectParams;

pub trait GraphQLClient {
    fn query<K>(&self, query: String) -> Pin<Box<dyn Future<Output = eyre::Result<Option<K>>>>>
    where
        K: for<'de> Deserialize<'de>;
}

pub type DynGraphQLClient = Arc<dyn GraphQLClient + Send + Sync>;

#[derive(Debug)]
pub struct DefaultGraphQLClient {
    client: gql_client::Client,
}

impl DefaultGraphQLClient {
    pub fn new(conn: &ConnectParams) -> Self {
        let token = general_purpose::URL_SAFE.encode(format!("{}:", conn.session_token));

        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Basic {}", token));

        Self {
            client: gql_client::Client::new_with_config(ClientConfig {
                endpoint: conn.url(),
                timeout: Some(1000),
                headers: Some(headers),
                proxy: None,
            }),
        }
    }
}

impl GraphQLClient for DefaultGraphQLClient {
    fn query<K>(&self, query: String) -> Pin<Box<dyn Future<Output = eyre::Result<Option<K>>>>>
    where
        Self: Sized,
        K: for<'de> Deserialize<'de>,
    {
        let res = self.client.query::<K>(&query);

        return Box::pin(res);
    }
}
