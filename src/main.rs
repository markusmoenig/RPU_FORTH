pub mod rpu;
pub mod buffer;
pub mod camera;
pub mod misc;
pub mod map;
pub mod tile;
pub mod world;
pub mod context;
pub mod palette;
pub mod scanner;
pub mod value;
pub mod sdf3d;

use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "embedded/"]
#[exclude = ".txt"]
#[exclude = ".DS_Store"]
pub struct Embedded;

pub mod prelude {
    pub use rayon::{slice::ParallelSliceMut, iter::{IndexedParallelIterator, ParallelIterator}};
    pub use serde::{Deserialize, Serialize};
    pub use maths_rs::prelude::*;
    pub use rustc_hash::FxHashMap;
    pub use fontdue::Font;
    pub use crate::Embedded;

    pub use crate::rpu::RPU;
    pub use crate::buffer::ColorBuffer;
    pub use crate::camera::Camera;
    pub use crate::misc::*;
    pub use crate::map::Map;
    pub use crate::tile::Tile;
    pub use crate::world::World;
    pub use crate::context::Context;
    pub use crate::palette::Palette;
    pub use crate::scanner::{Scanner, Token, TokenType};
    pub use crate::value::Value;
    pub use crate::sdf3d::*;
}

use prelude::*;

use std::fs::File;
use std::io::BufWriter;
use std::io;
use std::io::Write
;

use viuer::Config;

fn main() {

    let mut rpu = RPU::new();
    let mut buffer = ColorBuffer::new(1200, 800);

    println!("Welcome to the RPU Language Interpreter.");

    print!("rpu % ");
    io::stdout().flush().unwrap();

    for line in io::stdin().lines() {
        if let Some(line) = line.ok() {
            if line == "exit" {
                break;
            } else {
                let rc = rpu.process(line.trim().into(), &mut buffer);

                if rc.0 {
                    write_buffer(&buffer, Some("out.png"), true);
                }

                for l in rc.1 {
                    println!("{}", l);
                }
            }

            print!("rpu % ");
            io::stdout().flush().unwrap();
        } else {
            break;
        }
    }
}

fn write_buffer(buffer: &ColorBuffer, file_name: Option<&str>, terminal: bool) {

    // Write to file

    let data = buffer.to_u8_vec();

    if let Some(path) = file_name {
        if let Some(file) = File::create(path).ok() {
            let w = BufWriter::new(file);
            let mut encoder = png::Encoder::new(w, buffer.width as u32, buffer.height as u32);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            // Adding text chunks to the header
            encoder
                .add_text_chunk(
                    "RPU".to_string(),
                    "This image was procedurally generated by rpu-lang.org".to_string(),
                )
                .unwrap();

            if let Some(mut writer) = encoder.write_header().ok() {
                writer.write_image_data(&data).unwrap();
            }
        }
    }

    // Write to terminal

    if terminal {
        // Clear terminal first
        print!("{}[2J", 27 as char);

        let conf = Config {
            ..Default::default()
        };

        let mut img = image::DynamicImage::ImageRgba8(image::RgbaImage::new(buffer.width as u32, buffer.height as u32));

        if let Some(d) = img.as_mut_rgba8() {
            d.copy_from_slice(&data[..]);
        }

        viuer::print(&img, &conf).expect("Terminal output failed.");
    }
}