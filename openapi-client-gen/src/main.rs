use std::fs;

use clap::Parser;
use error::Error;

mod error;
mod file_tracker;
mod generate;
mod lang;
mod model;
mod operation;
mod parameter;
mod parse;
mod spec;
mod tag;
mod template;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Source open api or swagger spec file
    input_spec_file_path: String,
    /// Output directory for generated files
    target_directory: String,
    /// Generate a module instead of a crate
    #[arg(short, long)]
    module: bool,
}

fn main() -> Result<(), Error> {
    colog::init();

    let cli = Cli::parse();

    let contents = fs::read_to_string(cli.input_spec_file_path)?;
    let spec = spec::Spec::from_yaml_string(&contents)?;

    let mut file_tracker = file_tracker::FileTracker::new(cli.target_directory);

    generate::rust_hyper_legacy::generate(&spec, &mut file_tracker, cli.module)?;

    Ok(())
}
