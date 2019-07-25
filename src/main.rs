use std::fs;
use std::fs::File;
use std::io::prelude::*;

mod hitable;
mod ray;
pub mod vec3;

use hitable::*;
use ray::Ray;
use vec3::*;

fn main() -> std::io::Result<()> {
  let count = fs::read_dir("output/")?.count();
  let mut file = File::create(format!("output/hello_world_{}.ppm", count))?;

  let nx: isize = 200;
  let ny: isize = 100;

  let lower_left_corner = vec3(-2.0, -1.0, -1.0);
  let horizontal = vec3(4.0, 0.0, 0.0);
  let vertical = vec3(0.0, 2.0, 0.0);
  let origin = scalar(0.0);
  let world: Box<dyn Hitable> = Box::new(HitableList::new(vec![
    Box::new(Sphere::new(vec3(0.0, 0.0, -1.0), 0.5)),
    Box::new(Sphere::new(vec3(0.0, -100.5, -1.0), 100.0)),
  ]));

  file.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;
  for j in (0..ny).rev() {
    for i in 0..nx {
      let u = i as f64 / nx as f64;
      let v = j as f64 / ny as f64;

      let r = Ray::new(
        origin,
        lower_left_corner + scalar(u) * horizontal + scalar(v) * vertical,
      );

      let _p = r.point_at_parameter(2.0);
      let col = color(&r, &world);
      let ir = (255.99 * col.x) as isize;
      let ig = (255.99 * col.y) as isize;
      let ib = (255.99 * col.z) as isize;
      file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
    }
  }

  Ok(())
}

fn color(r: &Ray, world: &Box<dyn Hitable>) -> Vec3 {
  let mut rec = HitRecord::default();

  if world.hit(r, 0.0, std::f64::INFINITY, &mut rec) {
    return scalar(0.5) * vec3(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
  }

  let unit_direction = r.direction().unit();
  let t = 0.5 * (unit_direction.y + 1.0);
  scalar(1.0 - t) * scalar(1.0) + scalar(t) * vec3(0.5, 0.7, 1.0)
}
