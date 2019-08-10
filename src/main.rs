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
mod perlin;
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

  let nx: isize = 100 * factor;
  let ny: isize = 100 * factor;
  let ns: isize = quality;

  //let (camera, world) = light_sphere_scene(nx as f64 / ny as f64);
  let (camera, world) = cornell_smoke_scene(nx as f64 / ny as f64);

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

  file.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;
  let mut result = vec![scalar(0.0); (nx * ny) as usize];
  for r in outer_result.lock().unwrap().iter_mut() {
    for (col, other) in result.iter_mut().zip(r.as_ref().unwrap().iter()) {
      *col += *other
    }
  }
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
    let ir = (255.99 * col.x) as isize;
    let ig = (255.99 * col.y) as isize;
    let ib = (255.99 * col.z) as isize;

    file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
  }

  Ok(())
}

fn color(r: &Ray, world: &dyn Hitable, depth: isize) -> Vec3 {
  if let Some(mut rec) = world.hit(r, 0.001, std::f64::INFINITY) {
    // ON A DIFFUSE LIGHT, rec.u and rec.v don't matter, texture is constant
    // internally so it just returns color
    let emitted = rec
      .material
      .expect("Missing material somewhere")
      .emitted(rec.u, rec.v, rec.p);
    if depth < 50 {
      if let Some((attenuation, scattered)) = rec
        .material
        .take()
        .as_ref()
        .and_then(|m| m.scatter(r, &mut rec))
      {
        return emitted + attenuation * color(&scattered, world, depth + 1);
      }
    }
    return emitted;
  }

  //let unit_direction = r.direction().unit();
  //let t = 0.5 * (unit_direction.y + 1.0);
  //scalar(1.0 - t) * scalar(1.0) + scalar(t) * vec3(0.5, 0.7, 1.0)

  // skybox is black
  scalar(0.0)
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

pub fn random_scene(ratio: f64) -> (Camera, HitableList) {
  let mut list: Vec<Box<dyn Hitable>> = vec![];

  list.push(Box::new(Sphere::new(
    vec3(0.0, -1000.0, 0.0),
    1000.0,
    //DiffuseLight::new(Texture::new_constant(scalar(4.0))),
    Lambertian::new(Texture::new_constant(vec3(0.5, 0.4, 0.6))),
    //Lambertian::new(Texture::new_checker(
    //  Texture::new_constant(vec3(0.2, 0.3, 0.1)),
    //  Texture::new_constant(vec3(0.9, 0.9, 0.9)),
    //)),
  )));

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
        if choose_mat < 0.70 {
          // diffuse
          list.push(Box::new(Sphere::new(
            center,
            0.2,
            Lambertian::new_from_color(vec3(
              rng.gen::<f64>() * rng.gen::<f64>(),
              rng.gen::<f64>() * rng.gen::<f64>(),
              rng.gen::<f64>() * rng.gen::<f64>(),
            )),
          )));
        } else if choose_mat < 0.85 {
          //metal
          list.push(Box::new(Sphere::new(
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
          )));
        } else if choose_mat < 0.90 {
          // glass
          list.push(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
        } else {
          list.push(Box::new(Sphere::new(
            center,
            0.2,
            DiffuseLight::new(Texture::new_constant(vec3(
              4.0 * rng.gen::<f64>(),
              4.0 * rng.gen::<f64>(),
              4.0 * rng.gen::<f64>(),
            ))),
          )));
        }
      }
    }
  }

  list.push(Box::new(Sphere::new(
    vec3(0.0, 1.0, 0.0),
    1.0,
    Dielectric::new(1.5),
  )));
  list.push(Box::new(Sphere::new(
    vec3(0.0, 3.0, 0.0),
    1.0,
    //Dielectric::new(-1.5),
    DiffuseLight::new(Texture::new_constant(scalar(50.0))),
  )));
  list.push(Box::new(Sphere::new(
    vec3(-4.0, 1.0, 0.0),
    1.0,
    Lambertian::new_from_color(vec3(0.4, 0.2, 0.1)),
  )));
  list.push(Box::new(Sphere::new(
    vec3(4.0, 1.0, 0.0),
    1.0,
    Metal::new(vec3(0.7, 0.6, 0.5), 0.0),
  )));

  let list = HitableList::new(list);

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
      ratio,
      aperture,
      dist_to_focus,
    )
  };

  (camera, list)
}

/*
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

pub fn perlin_spheres_scene(ratio: f64) -> (Camera, HitableList) {
  let pertext = Texture::new_noise(5.0);

  let list: Vec<Sphere> = vec![
    Sphere::new(
      vec3(0.0, -1000.0, 0.0),
      1000.0,
      Lambertian::new(pertext.clone()),
    ),
    Sphere::new(vec3(0.0, 2.0, 0.0), 2.0, Lambertian::new(pertext)),
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

pub fn light_sphere_scene(ratio: f64) -> (Camera, HitableList) {
  let perlin = Lambertian::new(Texture::new_constant(vec3(0.0, 0.0, 0.5)));

  let light = DiffuseLight::new(Texture::new_constant(scalar(4.0)));

  let list: Vec<Sphere> = vec![
    Sphere::new(vec3(0.0, -10.0, 0.0), 10.0, perlin),
    Sphere::new(vec3(0.0, 10.0, 0.0), 10.0, light),
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
*/

pub fn simple_light_scene(ratio: f64) -> (Camera, HitableList) {
  let pertext = Texture::new_noise(4.0);
  let list: Vec<Box<dyn Hitable>> = vec![
    Box::new(Sphere::new(
      vec3(0.0, -1000.0, 0.0),
      1000.0,
      Lambertian::new(pertext.clone()),
    )),
    Box::new(Sphere::new(
      vec3(0.0, 2.0, 0.0),
      2.0,
      Lambertian::new(pertext),
    )),
    //Box::new(Sphere::new(
    //    vec3(0.0, 7.0, 0.0), 2.0, DiffuseLight::new(Texture::new_constant(scalar(4.0)))
    //)),
    Box::new(XYRect::new(
      3.0,
      5.0,
      1.0,
      3.0,
      -2.0,
      DiffuseLight::new(Texture::new_constant(scalar(4.0))),
    )),
  ];

  let list = HitableList::new(list);

  let lookfrom = vec3(30.0, 2.0, 3.0);
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

pub fn cornell_box_scene(ratio: f64) -> (Camera, HitableList) {
  let red = Lambertian::new(Texture::new_constant(vec3(0.65, 0.05, 0.05)));
  let white = Lambertian::new(Texture::new_constant(vec3(0.73, 0.73, 0.73)));
  let green = Lambertian::new(Texture::new_constant(vec3(0.12, 0.45, 0.15)));
  let light = DiffuseLight::new(Texture::new_constant(scalar(7.0)));

  let list: Vec<Box<dyn Hitable>> = vec![
    // Left wall
    Box::new(FlipNormals::new_yz(YZRect::new(
      0.0, 555.0, 0.0, 565.0, 555.0, green,
    ))),
    // Right wall
    Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)),
    // Light
    Box::new(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light)),
    // Ceiling
    Box::new(FlipNormals::new_xz(XZRect::new(
      0.0,
      555.0,
      0.0,
      555.0,
      555.0,
      white.clone(),
    ))),
    // Floor
    Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())),
    // Back wall
    Box::new(FlipNormals::new_xy(XYRect::new(
      0.0,
      555.0,
      0.0,
      555.0,
      555.0,
      white.clone(),
    ))),
    // Cuboids
    Box::new(Translate::new(
      Box::new(RotateY::new(
        Box::new(Cuboid::new(scalar(0.0), scalar(165.0), white.clone())),
        -18.0,
      )),
      vec3(130.0, 0.0, 65.0),
    )),
    Box::new(Translate::new(
      Box::new(RotateY::new(
        Box::new(Cuboid::new(
          scalar(0.0),
          vec3(165.0, 330.0, 165.0),
          white.clone(),
        )),
        15.0,
      )),
      vec3(265.0, 0.0, 295.0),
    )),
  ];

  let list = HitableList::new(list);

  let lookfrom = vec3(278.0, 278.0, -800.0);
  let lookat = vec3(278.0, 278.0, 0.0);

  let dist_to_focus = 10.0;
  let aperture = 0.0;
  let vfov = 40.0;

  (
    Camera::new(
      lookfrom,
      lookat,
      vec3(0.0, 1.0, 0.0),
      vfov,
      ratio,
      aperture,
      dist_to_focus,
    ),
    list,
  )
}

pub fn cornell_smoke_scene(ratio: f64) -> (Camera, HitableList) {
  let red = Lambertian::new(Texture::new_constant(vec3(0.65, 0.05, 0.05)));
  let white = Lambertian::new(Texture::new_constant(vec3(0.73, 0.73, 0.73)));
  let green = Lambertian::new(Texture::new_constant(vec3(0.12, 0.45, 0.15)));
  let light = DiffuseLight::new(Texture::new_constant(scalar(7.0)));

  let b2 = Box::new(Sphere::new(vec3(265.0, 150.0, 295.0), 150.0, white.clone()));

  let b3 = Box::new(Sphere::new(
    vec3(265.0, 150.0, 295.0),
    120.0,
    Dielectric::new(1.5),
  ));

  let b4 = Box::new(Sphere::new(
    vec3(265.0, 150.0, 295.0),
    110.0,
    Metal::new(scalar(1.0), 0.0),
  ));

  let list: Vec<Box<dyn Hitable>> = vec![
    // Left wall
    Box::new(FlipNormals::new_yz(YZRect::new(
      0.0, 555.0, 0.0, 565.0, 555.0, green,
    ))),
    // Right wall
    Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)),
    // Light
    Box::new(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light)),
    // Ceiling
    Box::new(FlipNormals::new_xz(XZRect::new(
      0.0,
      555.0,
      0.0,
      555.0,
      555.0,
      white.clone(),
    ))),
    // Floor
    Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())),
    // Back wall
    Box::new(FlipNormals::new_xy(XYRect::new(
      0.0,
      555.0,
      0.0,
      555.0,
      555.0,
      white.clone(),
    ))),
    //    Box::new(ConstantMedium::new(b1, 0.005, Texture::new_constant(scalar(0.5)))),
    Box::new(ConstantMedium::new(
      b2,
      0.01,
      Texture::new_constant(scalar(0.5)),
    )),
    b3,
    b4,
    //Box::new(Sphere::new(vec3(450.0,50.0,100.0), 50.0, Dielectric::new(1.5))),
    //Box::new(Sphere::new(vec3(350.0,50.0,200.0), 50.0, Metal::new(scalar(1.0),0.0))),
  ];

  let list = HitableList::new(list);

  let lookfrom = vec3(278.0, 278.0, -800.0);
  let lookat = vec3(278.0, 278.0, 0.0);

  let dist_to_focus = 10.0;
  let aperture = 0.0;
  let vfov = 40.0;

  (
    Camera::new(
      lookfrom,
      lookat,
      vec3(0.0, 1.0, 0.0),
      vfov,
      ratio,
      aperture,
      dist_to_focus,
    ),
    list,
  )
}
