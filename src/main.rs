use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
  let mut file = File::create("output/hello_world.ppm")?;

  let nx: isize = 200;
  let ny: isize = 100;
  file.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;
  for j in (0..ny).rev() {
    for i in 0..nx {
      let r = i as f64 / nx as f64;
      let g = j as f64 / ny as f64;
      let b = 0.2;
      let ir = (255.99 * r) as isize;
      let ig = (255.99 * g) as isize;
      let ib = (255.99 * b) as isize;
      file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
    }
  }

  Ok(())
}
