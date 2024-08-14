use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "webptimize", version = "0.1.0", about = "Convert image files to WebP", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Convert images to WebP")]
    Convert {
        #[arg(value_parser = clap::value_parser!(PathBuf), help = "Path to the file or directory to convert from")]
        input_path: PathBuf,

        #[arg(value_parser = clap::value_parser!(PathBuf), help = "Path to the output directory")]
        output_directory: Option<PathBuf>,

        #[arg(
            short,
            long,
            default_value_t = 80.0,
            help = "Sets the quality of the conversion"
        )]
        quality: f32,

        #[arg(
            short,
            long,
            default_value_t = false,
            help = "Wether to convert images in a lossless manner (slow)"
        )]
        lossless: bool,
    },
}
