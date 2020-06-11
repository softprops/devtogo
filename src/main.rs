mod new;
mod sync;
mod error;

use crate::error::StrErr;
use new::New;
use std::{env};
use structopt::StructOpt;
use sync::Sync;

/// A dev.to tool for the road 👩🏽‍💻🎒
///
/// Synchronizes local markdown files with dev.to articles and generates local templates.
#[derive(StructOpt)]
enum Opts {
    Sync(Sync),
    New(New),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match Opts::from_args() {
        Opts::Sync(args) => {
            sync::run(
                env::var("DEVTO_API_KEY")
                    .map_err(|_| StrErr("Please provide a DEVTO_API_KEY env variable".into()))?,
                args,
            )
            .await?
        }
        Opts::New(args) => new::run(args).await?,
    }
    Ok(())
}
