use rand::prelude::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

mod bvh;
mod camera;
mod hitable;
mod material;
mod ray;
mod texture;
pub mod vec3;

use bvh::*;
use camera::Camera;
use hitable::*;
use material::*;
use ray::Ray;
use texture::*;
use vec3::*;

fn main() -> std::io::Result<()> {
  let count = fs::read_dir("output/")?.count();
  let mut file = File::create(format!("output/hello_world_{}.ppm", count))?;

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

  let nx: isize = 200 * factor;
  let ny: isize = 100 * factor;
  let ns: isize = quality;

  let camera = {
    let lookfrom = vec3(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    Camera::new(
      lookfrom,
      lookat,
      vec3(0.0, 1.0, 0.0),
      20.0,
      nx as f64 / ny as f64,
      aperture,
      dist_to_focus,
    )
  };

  let world = random_scene();
  let (camera, world) = two_spheres_scene(nx as f64 / ny as f64);

  let world = Arc::new(world);
  let camera = Arc::new(camera);

  let core_count: isize = 4;
  let outer_result: Arc<Mutex<Vec<Option<_>>>> =
    Arc::new(Mutex::new(vec![None; core_count as usize]));

  let mut handles = vec![];
  for main_x in 0..core_count {
    let x = main_x as isize;
    let nx = nx;
    let ny = ny;
    let ns = ns;

    let camera = Arc::clone(&camera);
    let outer_result = Arc::clone(&outer_result);
    let world = Arc::clone(&world);

    let handle = thread::spawn(move || {
      let mut temp = world
        .list
        .iter()
        .map(|s| s as &Hitable)
        .collect::<Vec<&dyn Hitable>>();
      let bvh_world = BvhNode::new(&mut temp, 0.0, 0.0);

      let mut rng = rand::thread_rng();

      let mut result = vec![];

      for j in 0..ny / core_count {
        let j = j + (ny / core_count) * x;
        let j = ny - j;

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
          col = vec3(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
          let ir = (255.99 * col.x) as isize;
          let ig = (255.99 * col.y) as isize;
          let ib = (255.99 * col.z) as isize;

          result.push((ir, ig, ib));
        }
      }

      outer_result.lock().unwrap()[x as usize] = Some(result);
    });

    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  file.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;
  let mut result = vec![];
  for r in outer_result.lock().unwrap().iter_mut() {
    result.append(r.as_mut().unwrap());
  }
  for (ir, ig, ib) in result {
    file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
  }

  Ok(())
}

fn color(r: &Ray, world: &dyn Hitable, depth: isize) -> Vec3 {
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

pub fn random_scene() -> HitableList {
  let mut list: Vec<Sphere> = vec![];

  list.push(Sphere::new(
    vec3(0.0, -1000.0, 0.0),
    1000.0,
    Lambertian::new(Texture::new_checker(
      Texture::new_constant(vec3(0.2, 0.3, 0.1)),
      Texture::new_constant(vec3(0.9, 0.9, 0.9)),
    )),
  ));

  let mut rng = rand::thread_rng();

  for a in -11..11 {
    for b in -11..11 {
      let choose_mat: f64 = rng.gen();
      let center = vec3(
        a as f64 + 0.9 + rng.gen::<f64>(),
        0.2,
        b as f64 + 0.9 * rng.gen::<f64>(),
      );

      //continue;
      if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
        if choose_mat < 0.8 {
          // diffuse
          list.push(Sphere::new(
            center,
            0.2,
            Lambertian::new_from_color(vec3(
              rng.gen::<f64>() * rng.gen::<f64>(),
              rng.gen::<f64>() * rng.gen::<f64>(),
              rng.gen::<f64>() * rng.gen::<f64>(),
            )),
          ));
        } else if choose_mat < 0.95 {
          //metal
          list.push(Sphere::new(
            center,
            0.2,
            Metal::new(
              vec3(
                0.5 * (1.0 + rng.gen::<f64>()),
                0.5 * (1.0 + rng.gen::<f64>()),
                0.5 * (1.0 + rng.gen::<f64>()),
              ),
              0.5 * rng.gen::<f64>(),
            ),
          ));
        } else {
          // glass
          list.push(Sphere::new(center, 0.2, Dielectric::new(1.5)));
        }
      }
    }
  }

  list.push(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));
  /*
  list.push(Sphere::new(
    vec3(0.0, 1.0, 0.0),
    0.94,
    Dielectric::new(-1.5),
  ));
  */
  list.push(Sphere::new(
    vec3(-4.0, 1.0, 0.0),
    1.0,
    Lambertian::new_from_color(vec3(0.4, 0.2, 0.1)),
  ));
  list.push(Sphere::new(
    vec3(4.0, 1.0, 0.0),
    1.0,
    Metal::new(vec3(0.7, 0.6, 0.5), 0.0),
  ));

  HitableList::new(list)
}

pub fn two_spheres_scene(ratio: f64) -> (Camera, HitableList) {
  let checker = Lambertian::new(Texture::new_checker(
    Texture::new_constant(vec3(0.2, 0.3, 0.1)),
    Texture::new_constant(vec3(0.9, 0.9, 0.9)),
  ));

  let list: Vec<Sphere> = vec![
    Sphere::new(vec3(0.0, -10.0, 0.0), 10.0, checker.clone()),
    Sphere::new(vec3(0.0, 10.0, 0.0), 10.0, checker),
  ];

  let list = HitableList::new(list);

  let lookfrom = vec3(13.0, 2.0, 3.0);
  let lookat = vec3(0.0, 0.0, 0.0);

  let dist_to_focus = 10.0;
  let aperture = 0.0;

  (
    Camera::new(
      lookfrom,
      lookat,
      vec3(0.0, 1.0, 0.0),
      20.0,
      ratio,
      aperture,
      dist_to_focus,
    ),
    list,
  )
}
