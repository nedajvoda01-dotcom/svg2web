use clap::{Parser, Subcommand};

pub mod build;
pub mod analyze;
pub mod optimize;
pub mod parse;

#[derive(Parser)]
#[command(name = "svg2web")]
#[command(about = "Convert SVG to web components")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Build SVG to web component
    Build(build::BuildArgs),
    /// Parse SVG to JSON model
    Parse(parse::ParseArgs),
    /// Analyze SVG structure and complexity
    Analyze(analyze::AnalyzeArgs),
    /// Optimize SVG file
    Optimize(optimize::OptimizeArgs),
}

pub use build::{BuildArgs, BuildCommand};
pub use analyze::{AnalyzeArgs, AnalyzeCommand, OutputFormat};
pub use optimize::{OptimizeArgs, OptimizeCommand};
pub use parse::{ParseArgs, ParseCommand};
