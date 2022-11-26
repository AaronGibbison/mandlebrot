use crate::{ColourArrayRow, MAX_ITERATIONS, SIZE_X, SIZE_Y};

pub const COLOUR_SCALE: u32 = 4;
pub const COLOUR_BRIGHTNESS: u32 = 0xff >> COLOUR_SCALE;
pub const COLOUR_DEPTH: u32 = 0xff >> (8 - COLOUR_SCALE);

pub const COLOUR_R: u32 = COLOUR_DEPTH << (COLOUR_SCALE * 2);
pub const COLOUR_G: u32 = COLOUR_DEPTH << (COLOUR_SCALE * 1);
pub const COLOUR_B: u32 = COLOUR_DEPTH << (COLOUR_SCALE * 0);

pub fn colour(iterations: u32) -> (u8, u8, u8) {
  let colour = if MAX_ITERATIONS == iterations { 0 } else { 1 + (iterations - 1) % (COLOUR_DEPTH * 3) };

  (
    (COLOUR_BRIGHTNESS * (colour / 3)) as u8,
    (COLOUR_BRIGHTNESS * (colour & (COLOUR_DEPTH * 2)) ) as u8,
    (COLOUR_BRIGHTNESS * (colour % COLOUR_DEPTH)) as u8
  )
}

pub fn colour_row(set_colour: &mut [u8], px: usize, iterations: u32) -> () {
  let p = 3 * px;

  let (r, g, b) = colour(iterations);

  set_colour[p] = r;
  set_colour[p + 1] = g;
  set_colour[p + 2] = b;
}

pub fn colour_set(set_colour: &mut [u8], px: usize, py: usize, iterations: u32) -> () {
  let p = 3 * (py * SIZE_X + px);

  let (r, g, b) = colour(iterations);

  set_colour[p] = r;
  set_colour[p + 1] = g;
  set_colour[p + 2] = b;
}

pub fn colour_half(set_colour: &mut Vec<u8>, px: usize, py: usize, iterations: u32) -> () {
  let p = 3 * (py * SIZE_X + px);
  let rp = 3 * ((SIZE_Y - py - 1) * SIZE_X + px);

  let (r, g, b) = colour(iterations);

  set_colour[p] = r;
  set_colour[p + 1] = g;
  set_colour[p + 2] = b;

  set_colour[rp] = r;
  set_colour[rp + 1] = g;
  set_colour[rp + 2] = b;
}
