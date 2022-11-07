fn main() {
  // let SCALE: u8 = args[1].parse::<u8>().unwrap();
  const SCALE: u8 = 14;
  // let ACCURACY: u8 = args[2].parse::<u8>().unwrap();
  const ACCURACY: u8 = 15;

  // benchmark("gms", || generate_mandelbrot_set(SCALE, ACCURACY, escape::escape_time, mandelbrot::gms));
  // benchmark("bulb", || generate_mandelbrot_set(SCALE, ACCURACY, escape::escape_time_with_bulb, mandelbrot::gms));
  // benchmark("period", || generate_mandelbrot_set(SCALE, ACCURACY, escape::escape_time_with_period, mandelbrot::gms));
  // benchmark("bulb period", || generate_mandelbrot_set(SCALE, ACCURACY, escape::escape_time_with_bulb_period, mandelbrot::gms));
  // benchmark("bulb period half", || _generate_mandelbrot_set(mandelbrot::_gms_half, escape::escape_time_with_bulb_period, SCALE, ACCURACY));
  // benchmark("bulb period cluster", || _generate_mandelbrot_cluster(SCALE, ACCURACY));
  // benchmark("parallel bulb period", || _generate_mandelbrot_parallel(SCALE, ACCURACY));
  // benchmark("parallel2 bulb period", || generate_mandelbrot_parallel2(SCALE, ACCURACY));
  // benchmark("parallel3 bulb period", || generate_mandelbrot_parallel(SCALE, ACCURACY));
}
