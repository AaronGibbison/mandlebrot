use std::collections::VecDeque;
use std::thread;
use std::thread::JoinHandle;

use crate::mandelbrot::escape::*;
use crate::mandelbrot::colour::*;

pub fn _gms(algo: fn(f32, f32, u32) -> u32, accuracy: u8, size_x: usize, size_y: usize, x: (f32, f32), y: (f32, f32)) -> Vec<u8> {
  let max_iterations: u32 = 1 << accuracy;
  let scale_x = (x.1 - x.0) / (size_x - 1) as f32;
  let scale_y = (y.1 - y.0) / (size_y - 1) as f32;

  let mut mandelbrot_set = vec![0; size_y * size_x * 3];

  for py in 0..(size_y) {
    let y0 = scale_y * (py as f32) + y.0;

    for px in 0..size_x {
      let x0 = scale_x * (px as f32) + x.0;
      let iterations = algo(y0, x0, max_iterations);
      colour(&mut mandelbrot_set, max_iterations, size_x, size_y, px, py, iterations);
    }
  }

  return mandelbrot_set;
}

pub fn _gms_half(algo: fn(f32, f32, u32) -> u32, accuracy: u8, size_x: usize, size_y: usize, x: (f32, f32), y: (f32, f32)) -> Vec<u8> {
  let max_iterations: u32 = 1 << accuracy;
  let scale_x = (x.1 - x.0) / (size_x - 1) as f32;
  let scale_y = (y.1 - y.0) / (size_y - 1) as f32;
  let half_y = size_y / 2;

  let mut set_colour = vec![0; size_y * size_x * 3];

  for py in 0..half_y {
    let y0 = scale_y * (py as f32) + y.0;

    for px in 0..size_x {
      let x0 = scale_x * (px as f32) + x.0;

      let iterations = algo(y0, x0, max_iterations);
      colour_half(&mut set_colour, max_iterations, size_x, size_y, px, py, iterations);
    }
  }

  return set_colour;
}

pub fn gms_cluster(algo: fn(f32, f32, u32) -> u32, accuracy: u8, size_x: usize, size_y: usize, x: (f32, f32), y: (f32, f32)) -> Vec<u8> {
  const CLUSTER_SIZE: usize = 1 << 3;

  let max_iterations: u32 = 1 << accuracy;
  let scale_x = (x.1 - x.0) / (size_x - 1) as f32;
  let scale_y = (y.1 - y.0) / (size_y - 1) as f32;
  let half_y = size_y / 2;

  let mut set_colour = vec![0; size_y * size_x * 3];

  for cy in 0..half_y / CLUSTER_SIZE {
    for cx in 0..size_x / CLUSTER_SIZE {
      let c_y0 = scale_y * ((cy * CLUSTER_SIZE) as f32) + y.0;
      let c_x0 = scale_x * ((cx * CLUSTER_SIZE) as f32) + x.0;

      let c_iterations = algo(c_y0, c_x0, max_iterations);
      let c_colour = if c_iterations == max_iterations { 0 } else { 1 + (c_iterations - 1) % (COLOUR_R + COLOUR_G + COLOUR_B + 1) };

      let r = (COLOUR_BRIGHTNESS * (c_colour & COLOUR_R) >> 4) as u8;
      let g  = (COLOUR_BRIGHTNESS * (c_colour & COLOUR_G) >> 2) as u8;
      let b  = (COLOUR_BRIGHTNESS * (c_colour & COLOUR_B)) as u8;

      let mut is_boxed = true;

      // Left
      for py in 0..CLUSTER_SIZE {
        let y0 = scale_y * ((cy * CLUSTER_SIZE + py) as f32) + y.0;
        let c_iterations = algo(y0, c_x0, max_iterations);
        colour_half(&mut set_colour, max_iterations, size_x, size_y, cx * CLUSTER_SIZE, cy * CLUSTER_SIZE + py, c_iterations);

        let p = 3 * ((cy * CLUSTER_SIZE + py) * size_x + cx * CLUSTER_SIZE);
        is_boxed &= r == set_colour[p] &&
          g == set_colour[p + 1] &&
          b == set_colour[p + 2];
      }
      // Right
      for py in 0..CLUSTER_SIZE {
        let y0 = scale_y * ((cy * CLUSTER_SIZE + py) as f32) + y.0;
        let x0 = scale_x * (((cx + 1) * CLUSTER_SIZE - 1) as f32) + x.0;
        let c_iterations = algo(y0, x0, max_iterations);
        colour_half(&mut set_colour, max_iterations, size_x, size_y, (cx + 1) * CLUSTER_SIZE - 1, cy * CLUSTER_SIZE + py, c_iterations);

        let p = 3 * ((cy * CLUSTER_SIZE + py) * size_x + cx * CLUSTER_SIZE);
        is_boxed &= r == set_colour[p] &&
          g == set_colour[p + 1] &&
          b == set_colour[p + 2];
      }

      // Top
      for px in 1..(CLUSTER_SIZE - 1) {
        let x0 = scale_x * ((cx * CLUSTER_SIZE + px) as f32) + x.0;
        let c_iterations = algo(c_y0, x0, max_iterations);
        colour_half(&mut set_colour, max_iterations, size_x, size_y, cx * CLUSTER_SIZE + px, cy * CLUSTER_SIZE, c_iterations);

        let p = 3 * ((cy * CLUSTER_SIZE) * size_x + cx * CLUSTER_SIZE + px);
        is_boxed &= r == set_colour[p] &&
          g == set_colour[p + 1] &&
          b == set_colour[p + 2];
      }

      // Bottom
      for px in 1..(CLUSTER_SIZE - 1) {
        let y0 = scale_y * (((cy + 1) * CLUSTER_SIZE - 1) as f32) + y.0;
        let x0 = scale_x * ((cx * CLUSTER_SIZE + px) as f32) + x.0;
        let c_iterations = algo(y0, x0, max_iterations);
        colour_half(&mut set_colour, max_iterations, size_x, size_y, cx * CLUSTER_SIZE + px, (cy + 1) * CLUSTER_SIZE - 1, c_iterations);

        let p = 3 * (((cy + 1) * CLUSTER_SIZE - 1) * size_x + cx * CLUSTER_SIZE + px);
        is_boxed &= r == set_colour[p] &&
          g == set_colour[p + 1] &&
          b == set_colour[p + 2];
      }

      if is_boxed {
        for py in 1..(CLUSTER_SIZE - 1) {
          let cpy = cy * CLUSTER_SIZE + py;

          for px in 1..(CLUSTER_SIZE - 1) {
            let cpx = cx * CLUSTER_SIZE + px;

            let p = 3 * (cpy * size_x + cpx);
            let pr = 3 * ((size_y - cpy - 1) * size_x + cpx);

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
          let cpy = cy * CLUSTER_SIZE + py;
          let y0 = scale_y * (cpy as f32) + y.0;

          for px in 1..(CLUSTER_SIZE - 1) {
            let cpx = cx * CLUSTER_SIZE + px;
            let x0 = scale_x * (cpx as f32) + x.0;

            let iterations = algo(y0, x0, max_iterations);
            colour_half(&mut set_colour, max_iterations, size_x, size_y, cpx, cpy, iterations);
          }
        }
      }
    }
  }

  return set_colour;
}

pub fn _gms_parallel(algo: fn(f32, f32, u32) -> u32, accuracy: u8, size_x: usize, size_y: usize, x: (f32, f32), y: (f32, f32)) -> Vec<u8> {
  let max_iterations: u32 = 1 << accuracy;
  let scale_x = (x.1 - x.0) / (size_x - 1) as f32;
  let scale_y = (y.1 - y.0) / (size_y - 1) as f32;
  let half_y = size_y / 2;

  let mut set_colour = vec![0; size_y * size_x * 3];

  let (tx, px) = std::sync::mpsc::channel();

  {
    const THREADS: usize = 8;
    let x_slice = size_x / (THREADS);

    let mut threads: Vec<JoinHandle<()>> = vec!();

    for thread_index in 0..THREADS {
      let tx = tx.clone();

      let thread = std::thread::spawn(move || {
        for py in 0..half_y {
          let y0 = scale_y * (py as f32) + y.0;

          for px in (x_slice * thread_index)..(x_slice * (thread_index + 1)) {
            let x0 = scale_x * (px as f32) + x.0;
            let iterations = algo(y0, x0, max_iterations);
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

  for _ in 0..(half_y * size_x) {
    let (x, y, iterations) = px.recv().expect("Failed to receive iterations");
    colour_half(&mut set_colour, max_iterations, size_x, size_y, x, y, iterations);
  }

  return set_colour;
}

pub fn _gms_parallel2(algo: fn(f32, f32, u32) -> u32, accuracy: u8, size_x: usize, size_y: usize, x: (f32, f32), y: (f32, f32)) -> Vec<u8> {
  let max_iterations: u32 = 1 << accuracy;
  let scale_x = (x.1 - x.0) / (size_x - 1) as f32;
  let scale_y = (y.1 - y.0) / (size_y - 1) as f32;
  let half_y = size_y / 2;

  const THREADS: u8 = 8;

  let mut set_colour = (0..half_y)
      .into_iter()
      .enumerate()
      .map(|index|
          thread::spawn(move || {
            let mut colours = vec![0; size_x * 3];
            let py = index.1;
            let y0 = scale_y * (py as f32) + y.0;

            for px in 0..size_x {
              let x0 = scale_x * (px as f32) + x.0;
              let iterations = algo(y0, x0, max_iterations);
              colour(&mut colours, max_iterations, size_x, size_y, px, 0, iterations);
            }
            return colours;
          })
      )
      .fold(Vec::<u8>::with_capacity(size_y * size_x * 3),|mut acc, thread| {
        let mut result = thread.join().unwrap();
        acc.append(&mut result);

        return acc;
      });

  for colours in set_colour.clone().chunks(size_x * 3).rev() {
    set_colour.append(
      &mut colours.to_vec()
    );
  }

  return set_colour;
}

pub fn gms_parallel(algo: fn(f32, f32, u32) -> u32, accuracy: u8, size_x: usize, size_y: usize, x: (f32, f32), y: (f32, f32)) -> Vec<u8> {
  let max_iterations: u32 = 1 << accuracy;
  let scale_x = (x.1 - x.0) / (size_x - 1) as f32;
  let scale_y = (y.1 - y.0) / (size_y - 1) as f32;
  let half_y = size_y / 2;
  let px_row = size_x * 3;

  const THREADS: u8 = 8;

  let mut thread_enum = (0..half_y)
    .into_iter()
    .enumerate()
    .map(|index|
      thread::spawn(move || {
        let mut colours = vec![0; size_x * 3];
        let py = index.1;
        let y0 = scale_y * (py as f32) + y.0;

        for px in 0..size_x {
          let x0 = scale_x * (px as f32) + x.0;
          let iterations = algo(y0, x0, max_iterations);
          colour_row(&mut colours, max_iterations, px, iterations);
        }
        return colours;
      })
    );

  let mut set_colour = Vec::with_capacity(half_y);
  let mut active_threads = VecDeque::with_capacity(THREADS as usize);

  {
    for _ in 0..THREADS {
      let new_thread = thread_enum.next();
      if new_thread.is_some() {
        active_threads.push_back(new_thread);
      }
    }

    while !active_threads.is_empty() {
      let mut colours = active_threads.pop_front().unwrap().unwrap().join().unwrap();
      set_colour.append(&mut colours);

      let new_thread = thread_enum.next();
      if new_thread.is_some() {
        active_threads.push_back(new_thread);
      }
    }
  }

  for colours in set_colour.clone().chunks(px_row).rev() {
    set_colour.append(&mut colours.to_vec());
  }

  return set_colour;
}

pub fn _generate_mandelbrot_set(
  gms: fn(algo: fn(f32, f32, u32) -> u32, accuracy: u8, size_x: usize, size_y: usize, x: (f32, f32), y: (f32, f32)) -> Vec<u8>,
  escape: fn(f32, f32, u32) -> u32, scale: u8, accuracy: u8,
) -> Vec<u8> {
  let size = 1 << scale;
  return gms(escape, accuracy, ((size as f32) * 1.5) as usize, size, (-1.5, 0.5), (-1.0, 1.0));
}

pub fn _generate_mandelbrot_cluster(scale: u8, accuracy: u8) -> Vec<u8> {
  let size = 1 << scale;
  return gms_cluster(escape_time_with_bulb_period, accuracy, ((size as f32) * 1.5) as usize, size, (-2.0, 1.0), (-1.0, 1.0));
}

pub fn _generate_mandelbrot_parallel(scale: u8, accuracy: u8) -> Vec<u8> {
  let size = 1 << scale;
  return _gms_parallel(escape_time_with_bulb_period, accuracy, ((size as f32) * 1.5) as usize, size, (-2.0, 1.0), (-1.0, 1.0));
}

pub fn _generate_mandelbrot_parallel2(scale: u8, accuracy: u8) -> Vec<u8> {
  let size = 1 << scale;
  return _gms_parallel2(escape_time_with_bulb_period, accuracy, ((size as f32) * 1.5) as usize, size, (-2.0, 1.0), (-1.0, 1.0));
}

pub fn generate_mandelbrot_parallel(scale: u8, accuracy: u8) -> Vec<u8> {
  let size = 1 << scale;
  return gms_parallel(escape_time_with_bulb_period, accuracy, ((size as f32) * 1.5) as usize, size, (-2.0, 1.0), (-1.0, 1.0));
}
