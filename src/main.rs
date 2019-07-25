use std::fs::File;
use std::io::prelude::*;

mod ray;
pub mod vec3;

use ray::Ray;
use vec3::*;

fn main() -> std::io::Result<()> {
  let mut file = File::create("output/hello_world.ppm")?;

  let nx: isize = 200;
  let ny: isize = 100;

  let lower_left_corner = vec3(-2.0, -1.0, -1.0);
  let horizontal = vec3(4.0, 0.0, 0.0);
  let vertical = vec3(0.0, 2.0, 0.0);
  let origin = scalar(0.0);

  file.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;
  for j in (0..ny).rev() {
    for i in 0..nx {
      let u = i as f64 / nx as f64;
      let v = j as f64 / ny as f64;

      let r = Ray::new(
        origin,
        lower_left_corner + scalar(u) * horizontal + scalar(v) * vertical,
      );

      let col = color(&r);
      let ir = (255.99 * col.x) as isize;
      let ig = (255.99 * col.y) as isize;
      let ib = (255.99 * col.z) as isize;
      file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
    }
  }

  Ok(())
}

fn color(r: &Ray) -> Vec3 {
  if hit_sphere(vec3(0.0, 0.0, -1.0), 0.5, r) {
    return vec3(1.0, 0.0, 0.0);
  }

  let unit_direction = Vec3::make_unit_vector(r.direction());
  let t = 0.5 * (unit_direction.y + 1.0);
  scalar(1.0 - t) * scalar(1.0) + scalar(t) * vec3(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> bool {
  let oc = r.origin() - center;
  let a = r.direction().dot(r.direction());
  let b = 2.0 * oc.dot(r.direction());
  let c = oc.dot(oc) - radius * radius;
  let discriminant = b * b - 4.0 * a * c;
  discriminant > 0.0
}
