use std::sync::Arc;

use dagger_core::graphql_client::DefaultGraphQLClient;

use dagger_core::config::Config;
use dagger_core::engine::Engine as DaggerEngine;

use crate::errors::ConnectError;
use crate::gen::Query;
use crate::logging::{StdLogger, TracingLogger};
use crate::querybuilder::query;

pub type DaggerConn = Arc<Query>;

pub async fn connect() -> Result<DaggerConn, ConnectError> {
    let cfg = if cfg!(feature = "otel") {
        let cfg = Config::new(
            None,
            None,
            None,
            None,
            Some(Arc::new(TracingLogger::default())),
        );

        #[cfg(feature = "otel")]
        crate::logging::otel_logging().map_err(ConnectError::FailedToInstallOtelTracer)?;

        cfg
    } else {
        Config::new(None, None, None, None, Some(Arc::new(StdLogger::default())))
    };

    connect_opts(cfg).await
}

pub async fn connect_opts(cfg: Config) -> Result<DaggerConn, ConnectError> {
    let (conn, proc) = DaggerEngine::new()
        .start(&cfg)
        .await
        .map_err(ConnectError::FailedToConnect)?;

    Ok(Arc::new(Query {
        proc: proc.map(|p| Arc::new(p)),
        selection: query(),
        graphql_client: Arc::new(DefaultGraphQLClient::new(&conn)),
    }))
}

// Conn will automatically close on drop of proc

#[cfg(test)]
mod test {
    use super::connect;

    #[tokio::test]
    async fn test_connect() {
        let _ = connect().await.unwrap();
    }
}
