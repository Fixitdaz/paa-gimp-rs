# PAA-GIMP-RS

A high-performance Arma 3 PAA texture loader for GIMP 3.0.8+, written in Rust.

## Features
* Supports **DXT1/BC1** textures.
* Handles **LZO decompression** automatically.
* Integrated GIMP 3.0.8 Python plugin.

## Installation

### 1. The Converter (Rust)
1. Install [Rust](https://rustup.rs/).
2. Clone this repo.
3. Run `cargo build --release`.
4. Note the path to `target/release/paa-gimp-rs.exe`.

### 2. The GIMP Plugin
1. Locate your GIMP 3.0 plug-ins folder (usually `%APPDATA%\GIMP\3.0\plug-ins\`).
2. Copy the `file-paa-import` folder from this repo into that directory.
3. Open `file-paa-import.py` and update the `CONVERTER_PATH` to point to your compiled `.exe`.
4. Restart GIMP.

## License
MIT