#![allow(dead_code)]
#![allow(unused_imports)]
use rand::prelude::*;

use std::env;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;

mod bvh;
mod camera;
mod hitable;
mod material;
mod perlin;
mod ray;
mod texture;
pub mod vec3;
mod scenes;
mod onb;

use bvh::*;
use camera::Camera;
use hitable::*;
use material::*;
use ray::Ray;
use texture::*;
use vec3::*;
use scenes::*;

const SKYBOX_COLOR: f64 = 0.0;

fn main() -> std::io::Result<()> {
  /*
  let n = 1000000;
  let mut sum = 0.0;
  for _ in 0..n {
    let v = random_cosine_direction();
    sum += v.z * v.z * v.z / (v.z / std::f64::consts::PI);
  }
  println!("PI / 2 = {}", std::f64::consts::PI / 2.0);
  println!("Estimate = {}", sum / n as f64);
  return Ok(());
  */

  let args: Vec<String> = env::args().collect();
  let factor: isize = if args.len() > 1 {
    args[1].parse().unwrap()
  } else {
    1
  };

  let quality: isize = if args.len() > 2 {
    args[2].parse().unwrap()
  } else {
    100
  };

  let nx: isize = 100 * factor;
  let ny: isize = 100 * factor;
  let ns: isize = quality;

  //let (camera, world) = light_sphere_scene(nx as f64 / ny as f64);
  let (camera, world) = cornell_box_scene(nx as f64 / ny as f64);

  let world = Arc::new(world);
  let camera = Arc::new(camera);

  let threads: isize = 8;
  let outer_result: Arc<Mutex<Vec<Option<_>>>> = Arc::new(Mutex::new(vec![None; threads as usize]));

  let mut handles = vec![];
  for main_x in 0..threads {
    let x = main_x as isize;
    let nx = nx;
    let ny = ny;
    let ns = ns / threads;

    let camera = Arc::clone(&camera);
    let outer_result = Arc::clone(&outer_result);
    let world = Arc::clone(&world);

    let handle = thread::spawn(move || {
      let mut temp = world
        .list
        .iter()
        .map(|s| s.as_ref())
        .collect::<Vec<&dyn Hitable>>();
      let bvh_world = BvhNode::new(&mut temp, 0.0, 0.0);

      let mut rng = rand::thread_rng();

      let mut result: Vec<Vec3> = vec![];

      for j in (0..ny).rev() {
        for i in 0..nx {
          let mut col = scalar(0.0);

          // TODO: Stop early if all samples are very similar in color?
          for _ in 0..ns {
            let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
            let v = (j as f64 + rng.gen::<f64>()) / ny as f64;

            let r = camera.get_ray(u, v);
            let _p = r.point_at_parameter(2.0);
            let depth = 0;
            col += color(&r, &bvh_world, depth);
            //col += color(&r, &*world, depth);
          }

          col /= scalar(ns as f64);

          result.push(col);
        }
      }

      outer_result.lock().unwrap()[x as usize] = Some(result);
    });

    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  let mut result = vec![scalar(0.0); (nx * ny) as usize];
  for r in outer_result.lock().unwrap().iter_mut() {
    for (col, other) in result.iter_mut().zip(r.as_ref().unwrap().iter()) {
      *col += *other
    }
  }
  let mut buffer: Vec<u8> = vec![];

  for mut col in result {
    col /= scalar(threads as f64);
    col = vec3(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
    if col.x > 1.0 {
      col.x = 1.0;
    }
    if col.y > 1.0 {
      col.y = 1.0;
    }
    if col.z > 1.0 {
      col.z = 1.0;
    }
    let ir = (255.99 * col.x) as u8;
    let ig = (255.99 * col.y) as u8;
    let ib = (255.99 * col.z) as u8;

    buffer.push(ir);
    buffer.push(ig);
    buffer.push(ib);
  }

  let count = fs::read_dir("output/")?.count();
  let file_name = format!("output/hello_world_{}.png", count);

  image::save_buffer(file_name, &buffer, nx as u32, ny as u32, image::RGB(8)).unwrap();

  Ok(())
}

fn color(r: &Ray, world: &dyn Hitable, depth: isize) -> Vec3 {
  if let Some(mut rec) = world.hit(r, 0.001, std::f64::INFINITY) {
    // ON A DIFFUSE LIGHT, rec.u and rec.v don't matter, texture is constant
    // internally so it just returns color
    let emitted = rec
      .material
      .expect("Missing material somewhere")
      .emitted(&r, &rec, rec.u, rec.v, rec.p);
    if depth < 50 {
      let material = rec.material.take();
      if let Some((albedo, scattered, pdf)) = material
        .as_ref().and_then(|m| m.scatter(r, &mut rec))
      {
        let mut rng = rand::thread_rng();
        let on_light = vec3(
          213.0 + rng.gen::<f64>() * (343.0 - 213.0),
          554.0,
          227.0 + rng.gen::<f64>() * (332.0 - 227.0),
        );
        let to_light = on_light - rec.p;
        let distance_squared = to_light.squared_length();
        let to_light = to_light.unit();
        if to_light.dot(rec.normal) < 0.0 {
          return emitted;
        }
        let light_area = (343.0 - 213.0) * (332.0 - 227.0);
        let light_cosine = to_light.y.abs();
        if light_cosine < 0.000001 {
          return emitted;
        }
        let pdf = distance_squared / (light_cosine * light_area);
        let scattered = Ray::new(rec.p, to_light);

        return emitted
          + albedo
            * scalar(material.expect("Missing material in here")
              .scattering_pdf(r, &mut rec, &scattered))
            * color(&scattered, world, depth + 1) / scalar(pdf);
      }
    }
    return emitted;
  }

  //let unit_direction = r.direction().unit();
  //let t = 0.5 * (unit_direction.y + 1.0);
  //scalar(1.0 - t) * scalar(1.0) + scalar(t) * vec3(0.5, 0.7, 1.0)

  // skybox is black
  scalar(SKYBOX_COLOR)
}

pub fn random_in_unit_sphere() -> Vec3 {
  let mut p;
  let mut rng = rand::thread_rng();
  loop {
    p = scalar(2.0) * vec3(rng.gen(), rng.gen(), rng.gen()) - scalar(1.0);
    if p.squared_length() < 1.0 {
      break;
    }
  }
  p
}

pub fn random_on_unit_sphere() -> Vec3 {
  let mut p;
  let mut rng = rand::thread_rng();
  loop {
    p = scalar(2.0) * vec3(rng.gen(), rng.gen(), rng.gen()) - scalar(1.0);
    if p.squared_length() < 1.0 {
      break;
    }
  }
  p.unit()
}

fn pdf(_p: Vec3) -> f64 {
  1.0 / (4.0 * std::f64::consts::PI)
}

fn random_cosine_direction() -> Vec3 {
  let mut rng = rand::thread_rng();
  let r1 = rng.gen::<f64>();
  let r2 = rng.gen::<f64>();
  let z = (1.0 - r2).sqrt();
  let phi = 2.0 * std::f64::consts::PI * r1;
  let r2_sqrt = r2.sqrt();
  let x = phi.cos() * 2.0 * r2_sqrt;
  let y = phi.sin() * 2.0 * r2_sqrt;
  vec3(x, y, z)
}
