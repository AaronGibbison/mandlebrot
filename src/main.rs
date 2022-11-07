extern crate png;

mod file;
mod utility;
mod mandelbrot;

use file::img::*;
use utility::benchmark;
use mandelbrot::mandelbrot::*;

const FILE_SIZE: usize = 1024 * 1024;
const SCALE: u8 = 10;
const ACCURACY: u8 = 10;

fn main() {
  println!("Generating Set");
  let time_generate_set = std::time::SystemTime::now();
  let mandelbrot_set = generate_mandelbrot_parallel(SCALE, ACCURACY);
  println!("{:?}, {}, {} MB", time_generate_set.elapsed(), mandelbrot_set.len(), mandelbrot_set.len() / FILE_SIZE);

  println!("About to write set to file");
  let new_png = Img {
    colour_type: png::ColorType::RGB,
    width: ((1  << SCALE) as f32 * 1.5) as u32,
    height: 1 << SCALE,
    data: mandelbrot_set
  };

  let time_write_set = std::time::SystemTime::now();
  write_png("res/mandelbrot.png", new_png).expect("Was unable to write png");
  println!("Finished writing file. {:?}", time_write_set.elapsed());
}
