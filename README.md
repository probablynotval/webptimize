# webptimize

## Installation
### Linux/macOS
```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probablynotval/webptimizer/releases/download/v0.1.0/webptimize-installer.sh | sh
```

### Windows
```
powershell -c "irm https://github.com/probablynotval/webptimizer/releases/download/v0.1.0/webptimize-installer.ps1 | iex"
```

### Build from source
Supported Rust versions >= 1.79.0
```
git clone https://github.com/probablynotval/webptimizer.git
cd webptimizer
cargo build --release
```

## Usage
To convert a directory simply run:
```
webptimize convert /path/to/dir
```
Note: should work on Windows

Optionally you can specify the output directory and quality:
```
webptimize convert /path/to/dir /path/to/dir/webptimized -q 69
```

If for some reason you wish to convert losslessly it can be done using the -l flag:
```
webptimize convert /path/to/dir /path/to/dir/webptimized -l
```
Warning: this is quite slow.
If neither the output path or quality are specified the same directory will be used along with a quality of 80.


## Supported formats
This crate depends on [image](https://github.com/image-rs/image) for decoding. Refer to its [Supported Image Formats](https://github.com/image-rs/image?tab=readme-ov-file#supported-image-formats).

Currently animated GIFs (and WebPs) are unsupported but are planned in a future release.
AVIF is not supported.


## Notes
The goal of this project is to convert a bulk of images to WebP. That means it's currently not very fast at doing single file conversions.


## Todo
- [ ] Add support for animated GIFs
