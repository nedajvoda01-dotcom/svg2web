use clap::Parser;

use svg2web_cli::commands::{
	AnalyzeCommand, BuildCommand, Cli, Commands, OptimizeCommand, ParseCommand,
};
use svg2web_cli::logger::init_logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	init_logger();
	let cli = Cli::parse();

	match cli.command {
		Commands::Build(args) => {
			let cmd = BuildCommand::from_args(args);
			cmd.execute()?;
		}
		Commands::Parse(args) => {
			let cmd = ParseCommand::from_args(args);
			cmd.execute()?;
		}
		Commands::Analyze(args) => {
			let cmd = AnalyzeCommand::from_args(args);
			cmd.execute()?;
		}
		Commands::Optimize(args) => {
			let cmd = OptimizeCommand::from_args(args);
			cmd.execute()?;
		}
	}

	Ok(())
}
