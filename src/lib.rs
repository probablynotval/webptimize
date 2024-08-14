pub mod commands;

use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use console::Emoji;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use webp::Encoder;

const COUNTERCLOCKWISE: Emoji<'_, '_> = Emoji("🔄", "");
const MAGNIFYING_GLASS: Emoji<'_, '_> = Emoji("🔎", "");

pub fn convert_to_webp(
    input_path: &Path,
    output_directory: Option<&Path>,
    lossless: bool,
    quality: f32,
) -> Result<(), Box<dyn Error>> {
    let progress_spinner = progress_spinner(100, 100)?;

    let images = if input_path.is_dir() {
        let mut images: Vec<PathBuf> = Vec::new();
        let entries = fs::read_dir(input_path).unwrap();
        for entry in entries {
            if let Some(path) = entry.ok().map(|e| e.path()).filter(|p| p.is_file()) {
                progress_spinner
                    .set_message(format!("{} Validating: {:#?}", MAGNIFYING_GLASS, path));
                images.push(path);
            };
        }
        images
    } else {
        vec![input_path.to_owned()]
    };
    progress_spinner.finish_with_message(format!(
        "{} Successfuly validated {}",
        MAGNIFYING_GLASS,
        if images.len() == 1 { "file" } else { "files" }
    ));

    let progress_bar = progress_bar(images.len() as u64)?;

    images.par_iter().for_each(|image| {
        progress_bar.set_message(format!("{} Converting: {:#?}", COUNTERCLOCKWISE, image));
        let img = image::open(&image).unwrap();
        let encoder = Encoder::from_image(&img).unwrap();
        let encoded_webp = encoder.encode_simple(lossless, quality).unwrap();
        let output_file = determine_output_path(&image, output_directory).unwrap();

        fs::write(&output_file, &*encoded_webp).unwrap();
        progress_bar.inc(1);
    });
    progress_bar.finish_with_message(format!(
        "{} Successfully converted {} {}",
        COUNTERCLOCKWISE,
        images.len(),
        if images.len() == 1 { "file" } else { "files" }
    ));

    Ok(())
}

fn progress_spinner(length: u64, interval: u64) -> Result<ProgressBar, Box<dyn Error>> {
    let style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let bar = ProgressBar::new(length).with_style(style);
    bar.enable_steady_tick(Duration::from_millis(interval));
    bar.set_prefix("[1/2]");

    Ok(bar)
}

fn progress_bar(length: u64) -> Result<ProgressBar, Box<dyn Error>> {
    let style = ProgressStyle::with_template(
        "{prefix:.bold.dim}   {wide_msg}\n           {bar:50.green/white}{pos:>7}/{len:7}{elapsed}/{eta}",
    )
    .unwrap();
    let bar = ProgressBar::new(length).with_style(style);
    bar.set_prefix("[2/2]");

    Ok(bar)
}

fn determine_output_path(
    input_path: &Path,
    output_directory: Option<&Path>,
) -> Result<PathBuf, Box<dyn Error>> {
    let input_name = input_path.file_name().unwrap();
    match output_directory {
        Some(dir) if dir.is_dir() => {
            // dbg!("output_directory is some + dir");
            Ok(dir.join(input_name).with_extension("webp"))
        }
        Some(file) if file.is_file() => {
            // dbg!("output_directory is some + file");
            let parent_dir = file.parent().ok_or("Failed to get parent directory")?;
            Ok(parent_dir.join(input_name).with_extension("webp"))
        }
        None if input_path.is_dir() => {
            // dbg!("output_directory is none + input_path is dir");
            Ok(input_path.join(input_name).with_extension("webp"))
        }
        None if input_path.is_file() => {
            // dbg!("output_directory is none + input_path is file");
            let parent_dir = input_path
                .parent()
                .ok_or("Failed to get parent directory")?;
            Ok(parent_dir.join(input_name).with_extension("webp"))
        }
        _ => Err("Invalid path".into()),
    }
}
