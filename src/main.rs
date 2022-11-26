extern crate png;

mod file;
mod utility;
mod mandelbrot;

use file::img::*;
use mandelbrot::mandelbrot::*;
use mandelbrot::escape::*;

const FILE_SIZE_MB: usize = 1024 * 1024;
const SCALE: u8 = 12;
const ACCURACY: u8 = 15;

const MAX_ITERATIONS: u32 = 1 << ACCURACY;

const SIZE: usize = 1 << SCALE;
const SIZE_X: usize = ((SIZE as f32) * 1.5) as usize;
const SIZE_Y: usize = SIZE;
const COLOUR_ARRAY_SIZE: usize = SIZE_Y * SIZE_X * 3;
const COLOUR_ROW_SIZE: usize = SIZE_X * 3;
type ColourArrayRow = [u8; COLOUR_ROW_SIZE];

fn main() {
  println!("Generating Set");
  let time_generate_set = std::time::SystemTime::now();
  let mandelbrot_set = gms_parallel3(escape_time_with_bulb_period);
  println!("{:?}, {}, {} MB", time_generate_set.elapsed(), mandelbrot_set.len(), mandelbrot_set.len() / FILE_SIZE_MB);

  println!("About to write set to file");
  let new_png = Img {
    colour_type: png::ColorType::RGB,
    width: ((1  << SCALE) as f32 * 1.5) as u32,
    height: 1 << SCALE,
    data: mandelbrot_set
  };

  let time_write_set = std::time::SystemTime::now();
  // write_png("res/mandelbrot.png", new_png).expect("Was unable to write png");
  println!("Finished# writing file. {:?}", time_write_set.elapsed());
}
