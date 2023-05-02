use dagger_sdk::HostDirectoryOpts;
#[cfg(feature = "otel")]
use opentelemetry::global;
use tracing::Level;

#[cfg(feature = "otel")]
#[tracing::instrument]
#[tokio::main]
async fn main() -> eyre::Result<()> {
    let client = dagger_sdk::connect().await?;
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let span = tracing::span!(Level::INFO, "start main");
    let _enter = span.enter();

    let host_source_dir = client.host().directory_opts(
        "examples/build-the-application/app",
        HostDirectoryOpts {
            exclude: Some(vec!["node_modules".into(), "ci/".into()]),
            include: None,
        },
    );

    let source = client
        .container()
        .from("node:16")
        .with_mounted_directory("/src", host_source_dir.id().await?);

    let runner = source
        .with_workdir("/src")
        .with_exec(vec!["npm", "install"]);

    let test = runner.with_exec(vec!["npm", "test", "--", "--watchAll=false"]);

    let build_dir = test
        .with_exec(vec!["npm", "run", "build"])
        .directory("./build");

    let _ = build_dir.export("./build");

    let entries = build_dir.entries().await;

    println!("build dir contents: \n {:?}", entries);

    drop(_enter);

    global::shutdown_tracer_provider(); // sending remaining spans

    Ok(())
}
