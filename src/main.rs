mod commands;

use clap::Parser;
use commands::*;
use env_logger::Target;
use log::LevelFilter;
use webptimize::convert_to_webp;

fn main() {
    let _logger = env_logger::builder()
        .filter(None, LevelFilter::Debug)
        .target(Target::Stderr)
        .try_init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Convert {
            input_path,
            output_directory,
            quality,
            lossless,
        } => {
            convert_to_webp(input_path, output_directory.as_deref(), *lossless, *quality).unwrap();
        }
    }
}
