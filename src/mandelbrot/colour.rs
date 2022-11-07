use std::arch::x86_64::__m256i;

pub const COLOUR_SCALE: u32 = 4;
pub const COLOUR_BRIGHTNESS: u32 = 0xff >> COLOUR_SCALE;
pub const COLOUR_DEPTH: u32 = 0xff >> (8 - COLOUR_SCALE);

pub const COLOUR_R: u32 = COLOUR_DEPTH << (COLOUR_SCALE * 2);
pub const COLOUR_G: u32 = COLOUR_DEPTH << (COLOUR_SCALE * 1);
pub const COLOUR_B: u32 = COLOUR_DEPTH << (COLOUR_SCALE * 0);

pub fn colour_row(set_colour: &mut Vec<u8>, max_iterations: u32, px: usize, iterations: u32) -> () {
  let p = 3 * px;

  let colour = if max_iterations == iterations { 0 } else { 1 + (iterations - 1) % (COLOUR_R + COLOUR_G + COLOUR_B + 1) };

  set_colour[p] = (COLOUR_BRIGHTNESS * (colour & COLOUR_R) >> (COLOUR_SCALE * 2)) as u8;
  set_colour[p + 1] = (COLOUR_BRIGHTNESS * (colour & COLOUR_G) >> COLOUR_SCALE) as u8;
  set_colour[p + 2] = (COLOUR_BRIGHTNESS * (colour & COLOUR_B)) as u8;
}

pub fn colour(set_colour: &mut Vec<u8>, max_iterations: u32, size_x: usize, _size_y: usize, px: usize, py: usize, iterations: u32) -> () {
  let p = 3 * (py * size_x + px);

  let colour = if max_iterations == iterations { 0 } else { 1 + (iterations - 1) % (COLOUR_R + COLOUR_G + COLOUR_B + 1) };

  set_colour[p] = (COLOUR_BRIGHTNESS * (colour & COLOUR_R) >> (COLOUR_SCALE * 2)) as u8;
  set_colour[p + 1] = (COLOUR_BRIGHTNESS * (colour & COLOUR_G) >> COLOUR_SCALE) as u8;
  set_colour[p + 2] = (COLOUR_BRIGHTNESS * (colour & COLOUR_B)) as u8;
}

pub fn colour_half(set_colour: &mut Vec<u8>, max_iterations: u32, size_x: usize, size_y: usize, px: usize, py: usize, iterations: u32) -> () {
  let p = 3 * (py * size_x + px);
  let rp = 3 * ((size_y - py - 1) * size_x + px);

  let colour = if max_iterations == iterations { 0 } else { 1 + (iterations - 1) % (COLOUR_R + COLOUR_G + COLOUR_B + 1) };

  set_colour[p] = (COLOUR_BRIGHTNESS * (colour & COLOUR_R) >> (COLOUR_SCALE * 2)) as u8;
  set_colour[p + 1] = (COLOUR_BRIGHTNESS * (colour & COLOUR_G) >> COLOUR_SCALE) as u8;
  set_colour[p + 2] = (COLOUR_BRIGHTNESS * (colour & COLOUR_B)) as u8;

  set_colour[rp] = (COLOUR_BRIGHTNESS * (colour & COLOUR_R) >> (COLOUR_SCALE * 2)) as u8;
  set_colour[rp + 1] = (COLOUR_BRIGHTNESS * (colour & COLOUR_G) >> COLOUR_SCALE) as u8;
  set_colour[rp + 2] = (COLOUR_BRIGHTNESS * (colour & COLOUR_B)) as u8;
}
