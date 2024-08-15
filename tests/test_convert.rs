use webptimize::convert_to_webp;

use env_logger::Target;
use std::{
    env::{self},
    error::Error,
    fs::{self, File},
    io::Write,
};
use tempfile::tempdir;

#[cfg(test)]
fn init_logger() {
    let _logger = env_logger::builder()
        .is_test(true)
        .filter(None, log::LevelFilter::Debug)
        .target(Target::Stderr)
        .try_init();
}

#[test]
fn test_convert_img_to_webp_with_lossy_png() -> Result<(), Box<dyn Error>> {
    init_logger();

    let crate_root_dir = env::current_dir().unwrap();
    assert!(crate_root_dir.exists());
    let image_name = "wallhaven-rr2w91_3840x2160.png";
    let image_path = crate_root_dir.join("tests/images");
    let input_path = image_path.join(image_name);
    assert!(input_path.exists());

    let png_data = fs::read(&input_path).unwrap();

    let dir = tempdir()?;
    let input_png = dir.path().join(&input_path);
    let mut input_file = File::create(&input_png)?;
    input_file.write_all(&png_data)?;

    let output_webp = dir.path().join(image_name).with_extension("webp");
    File::create(&output_webp)?;

    let result = convert_to_webp(&input_png, Some(&output_webp), false, 80.0);
    assert!(result.is_ok());

    let webp_metadata = fs::metadata(&output_webp).unwrap();
    assert!(webp_metadata.len() > 0);

    let webp_data = fs::read(&output_webp).unwrap();
    assert!(!webp_data.is_empty());

    Ok(())
}

#[test]
fn test_convert_img_to_webp_with_lossy_directory() -> Result<(), Box<dyn Error>> {
    init_logger();

    let crate_root_dir = env::current_dir().unwrap();
    let image_directory = crate_root_dir.join("tests/images");
    assert!(image_directory.exists());
    assert!(image_directory.is_dir());

    let dir = tempdir()?;
    let output_directory = dir.path().join("some_dir");
    fs::create_dir(&output_directory)?;

    let result = convert_to_webp(&image_directory, Some(&output_directory), false, 85.0);
    assert!(result.is_ok());

    // Assert that the output WebP files contain data
    let entries = fs::read_dir(&output_directory).unwrap();
    for entry in entries {
        if let Some(image) = entry.ok().map(|f| f.path()) {
            let webp_metadata = fs::metadata(&image).unwrap();
            assert!(webp_metadata.len() > 0);
            let webp_data = fs::read(&image).unwrap();
            assert!(!webp_data.is_empty());
        }
    }

    Ok(())
}

#[test]
fn test_convert_img_to_webp_with_lossless_directory() -> Result<(), Box<dyn Error>> {
    let crate_root_dir = env::current_dir().unwrap();
    let image_directory = crate_root_dir.join("tests/images");
    assert!(image_directory.exists());
    assert!(image_directory.is_dir());

    let dir = tempdir()?;
    let output_directory = dir.path().join("some_dir");
    fs::create_dir(&output_directory)?;

    let result = convert_to_webp(&image_directory, Some(&output_directory), true, 100.0);
    assert!(result.is_ok());

    // Assert that the output WebP files contain data
    let entries = fs::read_dir(&output_directory).unwrap();
    for entry in entries {
        if let Some(image) = entry.ok().map(|f| f.path()) {
            let webp_metadata = fs::metadata(&image).unwrap();
            assert!(webp_metadata.len() > 0);
            let webp_data = fs::read(&image).unwrap();
            assert!(!webp_data.is_empty());
        }
    }

    Ok(())
}
