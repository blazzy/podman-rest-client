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

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// Source open api or swagger spec file
    input_spec_file: String,
    /// Output directory for generated files
    target_dir: String,
    /// Generate a module instead of a crate (optional)
    #[arg(long)]
    module: bool,
    /// Path to place files shared by all clients. Useful if generating multiple clients in the
    /// same project for different APIs or different versions of the same API (optional)
    #[arg(long)]
    common_dir: Option<String>,
    /// Module name for api files. If not specified, we guess based on the nearest src folder.
    #[arg(long)]
    api_module: Option<String>,
    /// Module name for common files. If not specified, we guess based on the nearest src folder.
    #[arg(long)]
    common_module: Option<String>,
    /// Do not generate the default client struct
    #[arg(long)]
    skip_default_client: bool,
    /// Assume all hashmap values can be nullable
    #[arg(long)]
    hash_maps_always_nullable: bool,
}

fn main() -> Result<(), Error> {
    colog::init();
    log::info!("Starting up");

    match app() {
        Err(err) => {
            log::error!("{}", err);
            Err(err)
        }
        ok => ok,
    }
}

fn app() -> Result<(), Error> {
    let cli = Cli::parse();

    let contents = fs::read_to_string(cli.input_spec_file)?;
    let spec = spec::Spec::from_yaml_string(
        &contents,
        &spec::Config {
            hash_maps_always_nullable: cli.hash_maps_always_nullable,
        },
    )?;

    generate::rust_hyper_legacy::generate(&mut generate::rust_hyper_legacy::GeneratorConfig {
        spec: &spec,
        target_dir: &cli.target_dir,
        is_module: cli.module,
        common_dir: cli.common_dir,
        api_module: cli.api_module,
        common_module: cli.common_module,
        skip_default_client: cli.skip_default_client,
    })?;

    Ok(())
}
