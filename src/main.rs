use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use s3_cdn::{
    config::S3CdnConfig,
    core::server::Server,
    frontend::http_frontend::{run, HttpFrontend},
    store::s3_store::S3Store,
};
use std::{sync::Arc, time::Duration};
use tokio_graceful_shutdown::{SubsystemBuilder, Toplevel};

use crate::logging::init_logging;

pub mod logging;

#[derive(Debug, Parser)]
#[command(
    name = "S3-CDN",
    help_template = r#"
{before-help}{name} {version} - {about}

{usage-heading} {usage}

{all-args}{after-help}

AUTHORS:
    {author}
"#,
    version,
    author
)]
#[command(about, long_about = None)]
#[command(propagate_version = true, arg_required_else_help = false)]
pub struct CommandLine {
    #[arg(short = 'P', long, global = true, help = "Port S3-CDN should run on")]
    pub port: Option<u16>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Start server", display_order = 10)]
    Server,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CommandLine::parse();

    println!("Hello, world!");
    println!("{:?}", cli);

    match cli.command {
        Commands::Server => {
            init_logging()?;

            let crate_version = clap::crate_version!();
            let git_revision = env!("S3_CDN_BUILD_GIT_HASH");

            tracing::info!("S3-CDN {}-{}", crate_version, git_revision);

            let mut config = S3CdnConfig::load().context("Cannot load Configuration")?;
            config.port = Some(cli.port.unwrap_or(config.port.unwrap_or(8080)));

            let server = {
                let store = Arc::new(S3Store::new(
                    &config.host,
                    &config.region,
                    &config.bucket,
                    &config.access_key_id,
                    &config.secret_access_key,
                ));
                let server = Arc::new(Server::new(store));
                let frontend = HttpFrontend::new(config.port.unwrap(), server);
                Toplevel::new(|s| async move {
                    s.start(SubsystemBuilder::new("Frontend", |h| run(frontend, h)));
                })
                .catch_signals()
                .handle_shutdown_requests(Duration::from_secs(5))
            };

            server.await?;
        }
    }

    Ok(())
}
