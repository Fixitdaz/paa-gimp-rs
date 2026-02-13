// author: fixitdaztv
// date: 2026-02-13
mod paa;

use anyhow::{Context, Result};
use binrw::BinRead;
use clap::Parser;
use std::fs::File;
use std::path::PathBuf;
use std::thread;
use image::{RgbaImage, ImageFormat};
use minilzo_rs::LZO;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long, default_value = "output.png")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("PAA-Rust Converter v1.9");
    println!("--------------------------------");
    println!("Opening: {:?}", args.input);

    let mut file = File::open(&args.input)
        .with_context(|| format!("Could not open file: {:?}", args.input))?;

    let paa_file = paa::PaaFile::read(&mut file)
        .context("Failed to parse PAA structure.")?;

    if let Some(main_mip) = paa_file.mipmaps.first() {
        let width = main_mip.width() as u32;
        let height = main_mip.height() as u32;
        let data = main_mip.data.clone();
        let output_path = args.output.clone();
        
        println!("Resolution: {}x{}", width, height);

        // Spawn a thread with a 8MB stack to handle decompression safely
        let handle = thread::Builder::new()
            .stack_size(8 * 1024 * 1024) 
            .spawn(move || -> Result<()> {
                let blocks_wide = (width + 3) / 4;
                let blocks_high = (height + 3) / 4;
                let expected_dxt_size = (blocks_wide * blocks_high * 8) as usize;

                let lzo = LZO::init().map_err(|e| anyhow::anyhow!("LZO init failed: {}", e))?;
                
                println!("Decompressing LZO stream...");
                let dxt1_data = if data.len() < expected_dxt_size {
                    lzo.decompress(&data, expected_dxt_size)
                        .map_err(|e| anyhow::anyhow!("LZO Decompression failed: {:?}", e))?
                } else {
                    data
                };

                println!("Decompressing BC1 (DXT1) to pixels...");
                let mut rgba_data = vec![0u8; (width * height * 4) as usize];
                squish::Format::Bc1.decompress(&dxt1_data, width as usize, height as usize, &mut rgba_data);

                let img = RgbaImage::from_raw(width, height, rgba_data)
                    .context("Invalid image buffer")?;
                img.save_with_format(&output_path, ImageFormat::Png)
                    .context("Failed to save PNG")?;

                Ok(())
            })?;

        handle.join().expect("Thread panicked")?;
        println!("--------------------------------");
        println!("SUCCESS: Saved to {:?}", args.output);
    } else {
        println!("Error: No image data found in {:?}", args.input);
    }

    Ok(())
}