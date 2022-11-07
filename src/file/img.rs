extern crate png;

use std::fs::File;
use std::io;

#[derive(Debug)]
pub struct Img {
    pub colour_type: png::ColorType,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

pub fn read_png(path: &str) -> io::Result<Img> {
    let decoder = png::Decoder::new(File::open(path)?);
    let (info, mut reader) = decoder.read_info()?;
    let mut img_data = vec![0; reader.output_buffer_size()];

    reader.next_frame(&mut img_data).expect("Unable to load png data");

    return Ok(
        Img {
            colour_type: info.color_type,
            width: info.width,
            height: info.height,
            data: img_data
        }
    );
}

pub fn write_png(path: &str, img: Img) -> io::Result<()> {
    let mut encoder = png::Encoder::new( File::create(path)?, img.width, img.height);
    encoder.set_color(img.colour_type);
    encoder.write_header()?.write_image_data(&img.data).expect("Failed to write image");

    return Ok(());
}