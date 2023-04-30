use dagger_core::logger::{DynLogger, Logger};
use tracing::Level;

pub fn default_logging() -> eyre::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    Ok(())
}

#[cfg(feature = "otel")]
pub fn otel_logging() -> eyre::Result<()> {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::Registry;

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("dagger_sdk")
        .install_simple();

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = Registry::default().with(telemetry);

    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        tracing::error!("failed to install global tracer: {}", e)
    }
    Ok(())
}

pub struct StdLogger {}

impl Default for StdLogger {
    fn default() -> Self {
        Self {}
    }
}

impl Logger for StdLogger {
    fn stdout(&self, output: &str) -> eyre::Result<()> {
        println!("{}", output);

        Ok(())
    }

    fn stderr(&self, output: &str) -> eyre::Result<()> {
        eprintln!("{}", output);

        Ok(())
    }
}

pub struct TracingLogger {}

impl Default for TracingLogger {
    fn default() -> Self {
        Self {}
    }
}

impl Logger for TracingLogger {
    fn stdout(&self, output: &str) -> eyre::Result<()> {
        tracing::info!(output = output, "dagger-sdk");

        Ok(())
    }

    fn stderr(&self, output: &str) -> eyre::Result<()> {
        tracing::warn!(output = output, "dagger-sdk");

        Ok(())
    }
}

pub struct AggregateLogger {
    pub loggers: Vec<DynLogger>,
}

impl Default for AggregateLogger {
    fn default() -> Self {
        Self {
            loggers: Vec::new(),
        }
    }
}

impl Logger for AggregateLogger {
    fn stdout(&self, output: &str) -> eyre::Result<()> {
        for logger in &self.loggers {
            logger.stdout(output).unwrap()
        }

        Ok(())
    }

    fn stderr(&self, output: &str) -> eyre::Result<()> {
        for logger in &self.loggers {
            logger.stderr(output).unwrap()
        }

        Ok(())
    }
}
