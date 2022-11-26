use std::collections::VecDeque;
use std::thread;
use std::thread::JoinHandle;

use crate::mandelbrot::colour::*;
use crate::{SIZE_X, SIZE_Y, COLOUR_ARRAY_SIZE, COLOUR_ROW_SIZE, MAX_ITERATIONS};

const HALF_Y: usize = SIZE_Y / 2;
const THREADS: usize = 8;

const GRID_X: (f32, f32) = (-2.0, 1.0);
const GRID_Y: (f32, f32) = (-1.0, 1.0);

const SCALE_X: f32 = (GRID_X.1 - GRID_X.0) / (SIZE_X - 1) as f32;
const SCALE_Y: f32 = (GRID_Y.1 - GRID_Y.0) / (SIZE_Y - 1) as f32;

type EscapeAlgorithm = fn(f32, f32) -> u32;

pub fn gms(algo: EscapeAlgorithm) -> Vec<u8> {
  let mut mandelbrot_set = vec![0; COLOUR_ARRAY_SIZE];
  for py in 0..(SIZE_Y) {
    let y0 = SCALE_Y * (py as f32) + GRID_Y.0;

    for px in 0..SIZE_X {
      let x0 = SCALE_X * (px as f32) + GRID_X.0;
      let iterations = algo(y0, x0);
      colour_set(&mut mandelbrot_set, px, py, iterations);
    }
  }

  return mandelbrot_set;
}

pub fn gms_half(algo: EscapeAlgorithm) -> Vec<u8> {
  let mut set_colour = vec![0; COLOUR_ARRAY_SIZE];

  for py in 0..HALF_Y {
    let y0 = SCALE_Y * (py as f32) + GRID_Y.0;

    for px in 0..SIZE_X {
      let x0 = SCALE_X * (px as f32) + GRID_X.0;

      let iterations = algo(y0, x0);
      colour_half(&mut set_colour, px, py, iterations);
    }
  }

  return set_colour;
}

pub fn gms_cluster(algo: EscapeAlgorithm) -> Vec<u8> {
  const CLUSTER_SIZE: usize = 1 << 3;

  let mut set_colour = vec![0; COLOUR_ARRAY_SIZE];

  for cy in 0..HALF_Y / CLUSTER_SIZE {
    let cy_offset_top = cy * CLUSTER_SIZE;
    let cy_offset_bottom = cy_offset_top + CLUSTER_SIZE - 1;

    for cx in 0..SIZE_X / CLUSTER_SIZE {
      let cx_offset_left = cx * CLUSTER_SIZE;
      let cx_offset_right = cx_offset_left + CLUSTER_SIZE - 1;

      let c_y0 = SCALE_Y * cy_offset_top as f32 + GRID_Y.0;
      let c_x0 = SCALE_X * cx_offset_left as f32 + GRID_X.0;

      let c_iterations = algo(c_y0, c_x0);
      let c_colour = if c_iterations == MAX_ITERATIONS { 0 } else { 1 + (c_iterations - 1) % (COLOUR_R + COLOUR_G + COLOUR_B + 1) };

      let r = (COLOUR_BRIGHTNESS * (c_colour & COLOUR_R) >> 4) as u8;
      let g = (COLOUR_BRIGHTNESS * (c_colour & COLOUR_G) >> 2) as u8;
      let b = (COLOUR_BRIGHTNESS * (c_colour & COLOUR_B)) as u8;

      let mut is_boxed = true;

      // Top
      for px in 0..CLUSTER_SIZE {
        let x0 = SCALE_X * ((cx_offset_left + px) as f32) + GRID_X.0;
        let c_iterations = algo(c_y0, x0);
        colour_half(&mut set_colour, cx_offset_left + px, cy_offset_top, c_iterations);

        let p = 3 * (cy_offset_top * SIZE_X + cx_offset_left + px);
        is_boxed &= r == set_colour[p] &&
          g == set_colour[p + 1] &&
          b == set_colour[p + 2];
      }

      // Bottom
      for px in 0..CLUSTER_SIZE {
        let y0 = SCALE_Y * cy_offset_bottom as f32 + GRID_Y.0;
        let x0 = SCALE_X * ((cx_offset_left + px) as f32) + GRID_X.0;
        let c_iterations = algo(y0, x0);
        colour_half(&mut set_colour, cx_offset_left + px, cy_offset_bottom, c_iterations);

        let p = 3 * (cy_offset_bottom * SIZE_X + cx_offset_left + px);
        is_boxed &= r == set_colour[p] &&
          g == set_colour[p + 1] &&
          b == set_colour[p + 2];
      }

      // Left
      for py in 1..(CLUSTER_SIZE - 1) {
        let y0 = SCALE_Y * ((cy_offset_top + py) as f32) + GRID_Y.0;
        let c_iterations = algo(y0, c_x0);
        colour_half(&mut set_colour, cx_offset_left, cy_offset_top + py, c_iterations);

        let p = 3 * ((cy_offset_top + py) * SIZE_X + cx_offset_left);
        is_boxed &= r == set_colour[p] &&
          g == set_colour[p + 1] &&
          b == set_colour[p + 2];
      }

      // Right
      for py in 1..(CLUSTER_SIZE - 1) {
        let y0 = SCALE_Y * ((cy_offset_top + py) as f32) + GRID_Y.0;
        let x0 = SCALE_X * cx_offset_right as f32 + GRID_X.0;
        let c_iterations = algo(y0, x0);
        colour_half(&mut set_colour, cx_offset_right, cy_offset_top + py, c_iterations);

        let p = 3 * ((cy_offset_top + py) * SIZE_X + cx_offset_right);
        is_boxed &= r == set_colour[p] &&
          g == set_colour[p + 1] &&
          b == set_colour[p + 2];
      }

      if is_boxed {
        for py in 1..(CLUSTER_SIZE - 1) {
          let cpy = cy_offset_top + py;

          for px in 1..(CLUSTER_SIZE - 1) {
            let cpx = cx_offset_left + px;

            let p = 3 * (cpy * SIZE_X + cpx);
            let pr = 3 * ((SIZE_Y - cpy - 1) * SIZE_X + cpx);

            set_colour[p] = r;
            set_colour[p + 1] = g;
            set_colour[p + 2] = b;

            set_colour[pr] = r;
            set_colour[pr + 1] = g;
            set_colour[pr + 2] = b;
          }
        }
      } else {
        for py in 1..(CLUSTER_SIZE - 1) {
          let cpy = cy_offset_top + py;
          let y0 = SCALE_Y * (cpy as f32) + GRID_Y.0;

          for px in 1..(CLUSTER_SIZE - 1) {
            let cpx = cx_offset_left + px;
            let x0 = SCALE_X * (cpx as f32) + GRID_X.0;

            let iterations = algo(y0, x0);
            colour_half(&mut set_colour, cpx, cpy, iterations);
          }
        }
      }
    }
  }

  return set_colour;
}

// No time difference
pub fn gms_cluster_simplified(algo: EscapeAlgorithm) -> Vec<u8> {
  const CLUSTER_SIZE: usize = 1 << 3;

  let mut set_colour = vec![0; COLOUR_ARRAY_SIZE];

  for cluster_y in 0..HALF_Y / CLUSTER_SIZE {
    let pixel_y_top = cluster_y * CLUSTER_SIZE;
    let pixel_y_bottom = pixel_y_top + CLUSTER_SIZE - 1;

    for cluster_x in 0..SIZE_X / CLUSTER_SIZE {
      let pixel_x_left = cluster_x * CLUSTER_SIZE;
      let pixel_x_right = pixel_x_left + CLUSTER_SIZE - 1;

      let cluster_y0 = SCALE_Y * pixel_y_top as f32 + GRID_Y.0;
      let cluster_x0 = SCALE_X * pixel_x_left as f32 + GRID_X.0;

      let cluster_iterations = algo(cluster_y0, cluster_x0);
      let (cluster_r, cluster_g, cluster_b) = colour(cluster_iterations);

      let mut is_boxed = true;

      // Top / Bottom
      for pixel_x in pixel_x_left..(pixel_x_left + CLUSTER_SIZE) {
        let x0 = SCALE_X * pixel_x as f32 + GRID_X.0;
        let y0 = SCALE_Y * pixel_y_bottom as f32 + GRID_Y.0;

        let top_iterations = algo(cluster_y0, x0);
        let bottom_iterations = algo(y0, x0);

        colour_half(&mut set_colour, pixel_x, pixel_y_top, top_iterations);
        colour_half(&mut set_colour, pixel_x, pixel_y_bottom, bottom_iterations);

        let pixel_top = 3 * (pixel_y_top * SIZE_X + pixel_x);
        is_boxed &= cluster_r == set_colour[pixel_top] &&
          cluster_g == set_colour[pixel_top + 1] &&
          cluster_b == set_colour[pixel_top + 2];

        let pixel_bottom = 3 * (pixel_y_bottom * SIZE_X + pixel_x);
        is_boxed &= cluster_r == set_colour[pixel_bottom] &&
          cluster_g == set_colour[pixel_bottom + 1] &&
          cluster_b == set_colour[pixel_bottom + 2];
      }

      // Left / Right
      for pixel_y in (pixel_y_top + 1)..(pixel_y_bottom) {
        let y0 = SCALE_Y * pixel_y as f32 + GRID_Y.0;
        let x0 = SCALE_X * pixel_x_right as f32 + GRID_X.0;

        let left_iterations = algo(y0, cluster_x0);
        let right_iterations = algo(y0, x0);

        colour_half(&mut set_colour, pixel_x_left, pixel_y, left_iterations);
        colour_half(&mut set_colour, pixel_x_right, pixel_y, right_iterations);

        let pixel_left = 3 * (pixel_y * SIZE_X + pixel_x_left);
        is_boxed &= cluster_r == set_colour[pixel_left] &&
          cluster_g == set_colour[pixel_left + 1] &&
          cluster_b == set_colour[pixel_left + 2];

        let pixel_right = 3 * (pixel_y * SIZE_X + pixel_x_right);
        is_boxed &= cluster_r == set_colour[pixel_right] &&
          cluster_g == set_colour[pixel_right + 1] &&
          cluster_b == set_colour[pixel_right + 2];
      }

      if is_boxed {
        for pixel_y in (pixel_y_top + 1)..(pixel_y_bottom) {
          for pixel_x in (pixel_x_left + 1)..(pixel_x_right) {
            let pixel = 3 * (pixel_y * SIZE_X + pixel_x);
            let reverse_pixel = 3 * ((SIZE_Y - pixel_y - 1) * SIZE_X + pixel_x);

            set_colour[pixel] = cluster_r;
            set_colour[pixel + 1] = cluster_g;
            set_colour[pixel + 2] = cluster_b;

            set_colour[reverse_pixel] = cluster_r;
            set_colour[reverse_pixel + 1] = cluster_g;
            set_colour[reverse_pixel + 2] = cluster_b;
          }
        }
      } else {
        for pixel_y in (pixel_y_top + 1)..(pixel_y_bottom) {
          let y0 = SCALE_Y * pixel_y as f32 + GRID_Y.0;

          for pixel_x in (pixel_x_left + 1)..(pixel_x_right) {
            let x0 = SCALE_X * pixel_x as f32 + GRID_X.0;

            let iterations = algo(y0, x0);
            colour_half(&mut set_colour, pixel_x, pixel_y, iterations);
          }
        }
      }
    }
  }

  return set_colour;
}

// Better with larger images - Make recursive
pub fn gms_cluster_checkered(algo: EscapeAlgorithm) -> Vec<u8> {
  const CLUSTER_SIZE: usize = 1 << 5;

  let mut mandelbrot_set_colour = vec![0; COLOUR_ARRAY_SIZE / 2];

  fn compute_cluster(algo: EscapeAlgorithm, mut set_colour: &mut Vec<u8>, pixel_y_top: usize, pixel_y_bottom: usize, pixel_x_left: usize, pixel_x_right: usize) {
    let cluster_y0 = SCALE_Y * pixel_y_top as f32 + GRID_Y.0;
    let cluster_x0 = SCALE_X * pixel_x_left as f32 + GRID_X.0;

    let cluster_iterations = algo(cluster_y0, cluster_x0);
    let colours = colour(cluster_iterations);

    let (cluster_r, cluster_g, cluster_b) = colours;

    let mut is_boxed = true;

    // Top / Bottom
    for pixel_x in pixel_x_left..(pixel_x_right + 1) {
      let x0 = SCALE_X * pixel_x as f32 + GRID_X.0;
      let y0 = SCALE_Y * pixel_y_bottom as f32 + GRID_Y.0;

      let top_iterations = algo(cluster_y0, x0);
      let bottom_iterations = algo(y0, x0);

      colour_set(&mut set_colour, pixel_x, pixel_y_top, top_iterations);
      colour_set(&mut set_colour, pixel_x, pixel_y_bottom, bottom_iterations);

      let pixel_top = 3 * (pixel_y_top * SIZE_X + pixel_x);
      is_boxed &= cluster_r == set_colour[pixel_top] &&
        cluster_g == set_colour[pixel_top + 1] &&
        cluster_b == set_colour[pixel_top + 2];

      let pixel_bottom = 3 * (pixel_y_bottom * SIZE_X + pixel_x);
      is_boxed &= cluster_r == set_colour[pixel_bottom] &&
        cluster_g == set_colour[pixel_bottom + 1] &&
        cluster_b == set_colour[pixel_bottom + 2];
    }

    // Left / Right
    for pixel_y in (pixel_y_top + 1)..(pixel_y_bottom) {
      let y0 = SCALE_Y * pixel_y as f32 + GRID_Y.0;
      let x0 = SCALE_X * pixel_x_right as f32 + GRID_X.0;

      let left_iterations = algo(y0, cluster_x0);
      let right_iterations = algo(y0, x0);

      colour_set(&mut set_colour, pixel_x_left, pixel_y, left_iterations);
      colour_set(&mut set_colour, pixel_x_right, pixel_y, right_iterations);

      let pixel_left = 3 * (pixel_y * SIZE_X + pixel_x_left);
      is_boxed &= cluster_r == set_colour[pixel_left] &&
        cluster_g == set_colour[pixel_left + 1] &&
        cluster_b == set_colour[pixel_left + 2];

      let pixel_right = 3 * (pixel_y * SIZE_X + pixel_x_right);
      is_boxed &= cluster_r == set_colour[pixel_right] &&
        cluster_g == set_colour[pixel_right + 1] &&
        cluster_b == set_colour[pixel_right + 2];
    }

    if is_boxed {
      for pixel_y in (pixel_y_top + 1)..(pixel_y_bottom) {
        for pixel_x in (pixel_x_left + 1)..(pixel_x_right) {
          let pixel = 3 * (pixel_y * SIZE_X + pixel_x);

          set_colour[pixel] = cluster_r;
          set_colour[pixel + 1] = cluster_g;
          set_colour[pixel + 2] = cluster_b;
        }
      }
    } else {
      if pixel_y_bottom - pixel_y_top < (1 << 2) {
        for pixel_y in (pixel_y_top + 1)..(pixel_y_bottom) {
          let y0 = SCALE_Y * pixel_y as f32 + GRID_Y.0;

          for pixel_x in (pixel_x_left + 1)..(pixel_x_right) {
            let x0 = SCALE_X * pixel_x as f32 + GRID_X.0;

            let iterations = algo(y0, x0);
            colour_set(&mut set_colour, pixel_x, pixel_y, iterations);
          }
        }
      } else {
        let pixel_y_mid = (pixel_y_top + pixel_y_bottom) / 2;
        let pixel_x_mid = (pixel_x_left + pixel_x_right) / 2;

        compute_cluster(algo, &mut set_colour, pixel_y_top + 1, pixel_y_mid, pixel_x_left + 1, pixel_x_mid);
        compute_cluster(algo, &mut set_colour, pixel_y_top + 1, pixel_y_mid, pixel_x_mid + 1, pixel_x_right - 1);
        compute_cluster(algo, &mut set_colour, pixel_y_mid + 1, pixel_y_bottom - 1, pixel_x_left + 1, pixel_x_mid);
        compute_cluster(algo, &mut set_colour, pixel_y_mid + 1, pixel_y_bottom - 1, pixel_x_mid + 1, pixel_x_right - 1);
      }
    }
  }

  for cluster_y in 0..HALF_Y / CLUSTER_SIZE {
    let pixel_y_top = cluster_y * CLUSTER_SIZE;
    let pixel_y_bottom = pixel_y_top + CLUSTER_SIZE - 1;

    for cluster_x in ((cluster_y % 2)..SIZE_X / CLUSTER_SIZE).step_by(2) {
      let pixel_x_left = cluster_x * CLUSTER_SIZE;
      let pixel_x_right = pixel_x_left + CLUSTER_SIZE - 1;

      compute_cluster(algo, &mut mandelbrot_set_colour, pixel_y_top, pixel_y_bottom, pixel_x_left, pixel_x_right)
    }
  }

  for cluster_y in 0..HALF_Y / CLUSTER_SIZE {
    let pixel_y_top = cluster_y * CLUSTER_SIZE;
    let pixel_y_bottom = pixel_y_top + CLUSTER_SIZE - 1;

    for cluster_x in (((cluster_y + 1) % 2)..SIZE_X / CLUSTER_SIZE).step_by(2) {
      let pixel_x_left = cluster_x * CLUSTER_SIZE;
      let pixel_x_right = pixel_x_left + CLUSTER_SIZE - 1;

      let cluster_y0 = SCALE_Y * pixel_y_top as f32 + GRID_Y.0;
      let cluster_x0 = SCALE_X * pixel_x_left as f32 + GRID_X.0;

      let cluster_iterations = algo(cluster_y0, cluster_x0);
      let colours = colour(cluster_iterations);

      {
        let mut is_checkered_boxed = true;

        if cluster_y != 0 {
          for pixel_x in pixel_x_left..pixel_x_right {
            let pixel_top = 3 * ((pixel_y_top - 1) * SIZE_X + pixel_x);
            is_checkered_boxed &=
              colours.0 == mandelbrot_set_colour[pixel_top] &&
                colours.1 == mandelbrot_set_colour[pixel_top + 1] &&
                colours.2 == mandelbrot_set_colour[pixel_top + 2];
          }
        }

        if cluster_y != (HALF_Y / CLUSTER_SIZE) - 1 {
          for pixel_x in pixel_x_left..pixel_x_right {
            let pixel_bottom = 3 * ((pixel_y_bottom + 1) * SIZE_X + pixel_x);
            is_checkered_boxed &=
              colours.0 == mandelbrot_set_colour[pixel_bottom] &&
                colours.1 == mandelbrot_set_colour[pixel_bottom + 1] &&
                colours.2 == mandelbrot_set_colour[pixel_bottom + 2];
          }
        }

        if cluster_x != 0 {
          for pixel_y in pixel_y_top..pixel_y_bottom {
            let pixel_left = 3 * (pixel_y * SIZE_X + pixel_x_left - 1);
            is_checkered_boxed &=
              colours.0 == mandelbrot_set_colour[pixel_left] &&
                colours.1 == mandelbrot_set_colour[pixel_left + 1] &&
                colours.2 == mandelbrot_set_colour[pixel_left + 2];
          }
        }

        if cluster_x != (SIZE_X / CLUSTER_SIZE) - 1 {
          for pixel_y in pixel_y_top..pixel_y_bottom {
            let pixel_right = 3 * (pixel_y * SIZE_X + pixel_x_right + 1);
            is_checkered_boxed &=
              colours.0 == mandelbrot_set_colour[pixel_right] &&
                colours.1 == mandelbrot_set_colour[pixel_right + 1] &&
                colours.2 == mandelbrot_set_colour[pixel_right + 2];
          }
        }

        if is_checkered_boxed {
          for pixel_y in pixel_y_top..(pixel_y_bottom + 1) {
            for pixel_x in pixel_x_left..(pixel_x_right + 1) {
              let pixel = 3 * (pixel_y * SIZE_X + pixel_x);
              mandelbrot_set_colour[pixel] = colours.0;
              mandelbrot_set_colour[pixel + 1] = colours.1;
              mandelbrot_set_colour[pixel + 2] = colours.2;
            }
          }

          continue;
        }
      }

      compute_cluster(algo, &mut mandelbrot_set_colour, pixel_y_top, pixel_y_bottom, pixel_x_left, pixel_x_right)
    }
  }

  for column in 0..HALF_Y {
    let column_index_start = COLOUR_ROW_SIZE * (HALF_Y - column - 1);
    let column_index_end = COLOUR_ROW_SIZE * (HALF_Y - column);

    mandelbrot_set_colour.extend_from_within(column_index_start..column_index_end);
  }

  return mandelbrot_set_colour;
}

// Extremely inefficient (= gms time...)
pub fn gms_parallel(algo: EscapeAlgorithm) -> Vec<u8> {
  let mut set_colour = vec![0; COLOUR_ARRAY_SIZE];

  let (tx, px) = std::sync::mpsc::channel();

  {
    const X_SLICE: usize = SIZE_X / THREADS;

    let mut threads: Vec<JoinHandle<()>> = vec!();

    for thread_index in 0..THREADS {
      let tx = tx.clone();

      let thread = thread::spawn(move || {
        for py in 0..HALF_Y {
          let y0 = SCALE_Y * (py as f32) + GRID_Y.0;

          for px in (X_SLICE * thread_index)..(X_SLICE * (thread_index + 1)) {
            let x0 = SCALE_X * (px as f32) + GRID_X.0;
            let iterations = algo(y0, x0);
            tx.send((px, py, iterations)).expect("Failed to send iterations");
          }
        }
      });

      threads.push(thread);
    }

    while !threads.is_empty() {
      threads.pop().unwrap().join().unwrap();
    }
  }

  for _ in 0..(HALF_Y * SIZE_X) {
    let (x, y, iterations) = px.recv().expect("Failed to receive iterations");
    colour_half(&mut set_colour, x, y, iterations);
  }

  return set_colour;
}

// Somewhat inefficient (~ 3/5 gms time)
pub fn gms_parallel2(algo: EscapeAlgorithm) -> Vec<u8> {
  let mut set_colour = (0..HALF_Y)
      .map(|index|
          thread::spawn(move || {
            let mut colours = [0; COLOUR_ROW_SIZE];
            let py = index;
            let y0 = SCALE_Y * (py as f32) + GRID_Y.0;

            for px in 0..SIZE_X {
              let x0 = SCALE_X * (px as f32) + GRID_X.0;
              let iterations = algo(y0, x0);
              colour_set(&mut colours, px, 0, iterations);
            }
            return (index, colours);
          })
      )
      .fold(vec![0; COLOUR_ARRAY_SIZE], |mut acc, thread| {
        let result = thread.join().unwrap();
        let offset: usize = result.0 * COLOUR_ROW_SIZE;

        for i in 0..COLOUR_ROW_SIZE {
          acc[offset + i] = result.1[i];
        }

        return acc;
      });

  for column in 0..(HALF_Y) {
    let column_index = COLOUR_ROW_SIZE * column;
    let opposite_column_index = COLOUR_ROW_SIZE * (SIZE_Y - column - 1);
    for row in 0..COLOUR_ROW_SIZE {
      set_colour[opposite_column_index + row] = set_colour[column_index + row]
    }
  }

  return set_colour;
}

// Mildly inefficient (~ gms_half / 2.5)
pub fn gms_parallel3(algo: EscapeAlgorithm) -> Vec<u8> {
  let mut thread_enum = (0..HALF_Y)
    .map(|index|
      thread::spawn(move || {
        let mut colours = [0; COLOUR_ROW_SIZE];
        let py = index;
        let y0 = SCALE_Y * (py as f32) + GRID_Y.0;

        for px in 0..SIZE_X {
          let x0 = SCALE_X * (px as f32) + GRID_X.0;
          let iterations = algo(y0, x0);
          colour_row(&mut colours, px, iterations);
        }
        return (index, colours);
      })
    );

  let mut set_colour = vec![0; COLOUR_ARRAY_SIZE];
  let mut active_threads = VecDeque::with_capacity(THREADS as usize);

  {
    for _ in 0..(THREADS + 1) {
      let new_thread = thread_enum.next();
      if new_thread.is_some() {
        active_threads.push_back(new_thread.unwrap());
      }
    }

    while !active_threads.is_empty() {
      let result = active_threads.pop_front().unwrap().join().unwrap();
      let offset: usize = result.0 * COLOUR_ROW_SIZE;

      for i in 0..COLOUR_ROW_SIZE {
        set_colour[offset + i] = result.1[i];
      }

      let new_thread = thread_enum.next();
      if new_thread.is_some() {
        active_threads.push_back(new_thread.unwrap());
      }
    }
  }

  for column in 0..(HALF_Y) {
    let column_index = COLOUR_ROW_SIZE * column;
    let opposite_column_index = COLOUR_ROW_SIZE * (SIZE_Y - column - 1);
    for row in 0..COLOUR_ROW_SIZE {
      set_colour[opposite_column_index + row] = set_colour[column_index + row]
    }
  }

  return set_colour;
}

pub fn gms_parallel_scoped(algo: EscapeAlgorithm) -> Vec<u8> {
  let mut set_colour = vec![0; COLOUR_ARRAY_SIZE / 2];

  thread::scope(|scope| {
    let threads = set_colour
      .chunks_mut(COLOUR_ROW_SIZE)
      .enumerate()
      .rev()
      .map(|(py, chunk)|
        scope.spawn(move || {
          let y0 = SCALE_Y * (py as f32) + GRID_Y.0;

          for px in 0..SIZE_X {
            let x0 = SCALE_X * (px as f32) + GRID_X.0;
            let iterations = algo(y0, x0);
            colour_row(chunk, px, iterations);
          }
        })
      );

    for _ in threads {}
  });

  for column in 0..HALF_Y {
    let column_index_start = COLOUR_ROW_SIZE * (HALF_Y - column - 1);
    let column_index_end = COLOUR_ROW_SIZE * (HALF_Y - column);

    set_colour.extend_from_within(column_index_start..column_index_end);
  }

  return set_colour;
}

pub fn gms_parallel_scoped_pixel(algo: EscapeAlgorithm) -> Vec<u8> {
  const CHUNKS_PER_ROW: usize = 4;
  const CHUNK_SIZE: usize = COLOUR_ROW_SIZE / CHUNKS_PER_ROW;
  const CHUNK_PIXELS: usize = CHUNK_SIZE / 3;

  println!("{} KB", CHUNK_SIZE / 1024);

  let mut set_colour = vec![0; COLOUR_ARRAY_SIZE / 2];
  thread::scope(|scope| {
    let mut threads = set_colour
      .chunks_mut(CHUNK_SIZE)
      .enumerate()
      .map(|(index, chunk)|
        scope.spawn(move || {
          let py = index / CHUNKS_PER_ROW;
          let y0 = SCALE_Y * (py as f32) + GRID_Y.0;

          let px_offset = CHUNK_PIXELS * (index % CHUNKS_PER_ROW);
          for px in 0..CHUNK_PIXELS {
            let x0 = SCALE_X * (px_offset + px) as f32 + GRID_X.0;
            let iterations = algo(y0, x0);
            colour_row(chunk, px, iterations);
          }
        })
      );

    for _ in threads {}
  });

  for column in 0..HALF_Y {
    let column_index_start = COLOUR_ROW_SIZE * (HALF_Y - column - 1);
    let column_index_end = COLOUR_ROW_SIZE * (HALF_Y - column);

    set_colour.extend_from_within(column_index_start..column_index_end);
  }

  return set_colour;
}

pub fn gms_parallel_cluster(algo: EscapeAlgorithm) -> Vec<u8> {
  const CLUSTER_SIZE: usize = 1 << 3;

  let mut set_colour = vec![0; COLOUR_ARRAY_SIZE / 2];

  thread::scope(|scope| {
    let threads = set_colour
      .chunks_mut(COLOUR_ROW_SIZE)
      .enumerate()
      .rev()
      .map(|(py, chunk)|
        scope.spawn(move || {
          let y0 = SCALE_Y * (py as f32) + GRID_Y.0;

          for px in 0..SIZE_X {
            let x0 = SCALE_X * (px as f32) + GRID_X.0;
            let iterations = algo(y0, x0);
            colour_row(chunk, px, iterations);
          }
        })
      );

    for _ in threads {}
  });

  for column in 0..HALF_Y {
    let column_index_start = COLOUR_ROW_SIZE * (HALF_Y - column - 1);
    let column_index_end = COLOUR_ROW_SIZE * (HALF_Y - column);

    set_colour.extend_from_within(column_index_start..column_index_end);
  }

  return set_colour;
}