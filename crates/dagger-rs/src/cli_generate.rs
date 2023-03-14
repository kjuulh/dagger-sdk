use std::io::Write;
use std::sync::Arc;

use clap::{Arg, ArgMatches};
use dagger_codegen::generate;
use dagger_codegen::rust::RustGenerator;
use dagger_core::config::Config;
use dagger_core::engine::Engine;
use dagger_core::session::Session;

#[allow(dead_code)]
pub struct GenerateCommand;

#[allow(dead_code)]
impl GenerateCommand {
    pub fn new_cmd() -> clap::Command {
        clap::Command::new("generate").arg(Arg::new("output").long("output"))
    }

    pub async fn exec(arg_matches: &ArgMatches) -> eyre::Result<()> {
        let cfg = Config::default();
        let (conn, _proc) = Engine::new().start(&cfg).await?;
        let session = Session::new();
        let req = session.start(&cfg, &conn)?;
        let schema = session.schema(req).await?;
        let code = generate(
            schema.into_schema().schema.unwrap(),
            Arc::new(RustGenerator {}),
        )?;

        if let Some(output) = arg_matches.get_one::<String>("output") {
            let mut file = std::fs::File::create(output)?;
            file.write(code.as_bytes())?;
        } else {
            println!("{}", code);
        }

        Ok(())
    }
}
