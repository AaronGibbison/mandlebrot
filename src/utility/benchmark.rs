pub fn benchmark<T>(name: &str, algo: fn() -> T) {
  const ITERATIONS: u32 = 100;

  let now = std::time::SystemTime::now();
  print!("{:?} Executing {}... ", now, name);

  for _ in 0..ITERATIONS {
    algo();
  }

  println!("{:?}", now.elapsed().expect("") / ITERATIONS );
}
