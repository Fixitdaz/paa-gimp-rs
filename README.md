# PAA-GIMP-RS

A high-performance Arma 3 PAA texture loader for GIMP 3.0.8+, written in Rust. This tool handles the complex task of decompressing PAA files (LZO + BC1/DXT1) and bridging them directly into your GIMP workspace.

## ⚠️ Disclaimer: Work in Progress
This project is currently a **Work in Progress (WIP)**. 
* You may encounter bugs, performance issues, or errors with specific PAA formats.
* It has been primarily tested with **DXT1/BC1** textures.
* Use at your own risk. Bug reports and contributions are welcome!

## Features
* **LZO Decompression**: Automatically handles Arma's compressed data streams.
* **BC1/DXT1 Support**: Decompresses standard Arma 3 textures.
* **Threaded Processing**: Uses a dedicated 8MB stack thread to prevent crashes on high-res (2K/4K) textures.
* **GIMP 3.0.8 Integration**: A Python 3 bridge using GObject Introspection.

## Installation

### 1. Build the Converter
1. Install [Rust](https://rustup.rs/).
2. Clone this repository.
3. Open a terminal in the folder and run:`cargo build --release`.
4. Note the path to `target/release/paa-gimp-rs.exe`.

### 2. The GIMP Plugin
1. Locate your GIMP 3.0 plug-ins folder (usually `%APPDATA%\GIMP\3.0\plug-ins\`).
2. Copy the `file-paa-import` folder from this repo into that directory.
3. Open `file-paa-import.py` and update the `CONVERTER_PATH` to point to your compiled `.exe`.
4. Restart GIMP.

## License
MIT
