use rand::prelude::*;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

mod camera;
mod hitable;
mod material;
mod ray;
pub mod vec3;

use camera::Camera;
use hitable::*;
use material::*;
use ray::Ray;
use vec3::*;

fn main() -> std::io::Result<()> {
  let count = fs::read_dir("output/")?.count();
  let mut file = File::create(format!("output/hello_world_{}.ppm", count))?;

  let nx: isize = 200;
  let ny: isize = 100;
  let ns: isize = 100;

  let world: Box<dyn Hitable> = Box::new(HitableList::new(vec![
    Box::new(Sphere::new(
      vec3(0.0, 0.0, -1.0),
      0.5,
      Box::new(Lambertian::new(vec3(0.1, 0.2, 0.5))),
    )),
    Box::new(Sphere::new(
      vec3(0.0, -100.5, -1.0),
      100.0,
      Box::new(Lambertian::new(vec3(0.8, 0.8, 0.0))),
    )),
    Box::new(Sphere::new(
      vec3(1.0, 0.0, -1.0),
      0.5,
      Box::new(Metal::new(vec3(0.8, 0.6, 0.2), 1.0)),
    )),
    Box::new(Sphere::new(
      vec3(-1.0, 0.0, -1.0),
      0.5,
      Box::new(Dielectric::new(1.5)),
    )),
    Box::new(Sphere::new(
      vec3(-1.0, 0.0, -1.0),
      -0.45,
      Box::new(Dielectric::new(1.5)),
    )),
  ]));
  let camera = Camera::specific();
  let mut rng = rand::thread_rng();

  file.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;
  for j in (0..ny).rev() {
    for i in 0..nx {
      let mut col = scalar(0.0);

      for _ in 0..ns {
        let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
        let v = (j as f64 + rng.gen::<f64>()) / ny as f64;

        let r = camera.get_ray(u, v);
        let _p = r.point_at_parameter(2.0);
        let depth = 0;
        col += color(&r, &world, depth);
      }

      col /= scalar(ns as f64);
      col = vec3(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
      let ir = (255.99 * col.x) as isize;
      let ig = (255.99 * col.y) as isize;
      let ib = (255.99 * col.z) as isize;
      file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
    }
  }

  Ok(())
}

fn color(r: &Ray, world: &Box<dyn Hitable>, depth: isize) -> Vec3 {
  if let Some(mut rec) = world.hit(r, 0.001, std::f64::INFINITY) {
    if depth < 50 {
      if let Some((attenuation, scattered)) = rec
        .material
        .take()
        .as_ref()
        .and_then(|m| m.scatter(r, &mut rec))
      {
        return attenuation * color(&scattered, world, depth + 1);
      }
    }
    return scalar(0.0);
  }

  let unit_direction = r.direction().unit();
  let t = 0.5 * (unit_direction.y + 1.0);
  scalar(1.0 - t) * scalar(1.0) + scalar(t) * vec3(0.5, 0.7, 1.0)
}

pub fn random_in_unit_sphere() -> Vec3 {
  let mut p;
  let mut rng = rand::thread_rng();
  loop {
    p = scalar(2.0) * vec3(rng.gen(), rng.gen(), rng.gen()) - scalar(1.0);
    if p.squared_length() >= 1.0 {
      break;
    }
  }
  p
}
