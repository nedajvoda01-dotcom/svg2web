use clap::Parser;

use svg2web_cli::commands::{run, Cli};
use svg2web_cli::logger::init_logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	init_logger();
	let cli = Cli::parse();
	run(cli).await
}
