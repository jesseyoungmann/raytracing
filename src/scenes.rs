use rand::prelude::*;

use crate::bvh::*;
use crate::camera::Camera;
use crate::hitable::*;
use crate::material::*;
use crate::ray::Ray;
use crate::texture::*;
use crate::vec3::*;

/*
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
*/

pub fn cornell_box_scene(ratio: f64) -> (Camera, HitableList) {
  let red = Lambertian::new(Texture::new_constant(vec3(0.65, 0.05, 0.05)));
  let white = Lambertian::new(Texture::new_constant(vec3(0.73, 0.73, 0.73)));
  let green = Lambertian::new(Texture::new_constant(vec3(0.12, 0.45, 0.15)));
  let light = DiffuseLight::new(Texture::new_constant(scalar(15.0)));

  let list: Vec<Box<dyn Hitable>> = vec![
    // Left wall
    Box::new(FlipNormals::new_yz(YZRect::new(
      0.0, 555.0, 0.0, 565.0, 555.0, green,
    ))),
    // Right wall
    Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)),
    // Light
    Box::new(FlipNormals::new_xz(XZRect::new(
      213.0,
      343.0,
      227.0,
      332.0,
      554.0,
      light.clone(),
    ))),
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

/*
pub fn cornell_smoke_scene(ratio: f64) -> (Camera, HitableList) {
  let red = Lambertian::new(Texture::new_constant(vec3(0.65, 0.05, 0.05)));
  let white = Lambertian::new(Texture::new_constant(vec3(0.73, 0.73, 0.73)));
  let green = Lambertian::new(Texture::new_constant(vec3(0.12, 0.45, 0.15)));
  let light = DiffuseLight::new(Texture::new_constant(scalar(7.0)));

  // WRAPPED IN VOLUMETRIC
  //let b2 = Box::new(Sphere::new(vec3(265.0, 150.0, 295.0), 150.0, white.clone()));

  let b3 = Box::new(Sphere::new(
    vec3(277.5, 150.0, 295.0),
    120.0,
    Dielectric::new(1.5),
  ));

  let b4 = Box::new(Sphere::new(
    vec3(277.5, 150.0, 295.0),
    118.0,
    Metal::new(scalar(1.0),0.0),
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
    //Box::new(ConstantMedium::new(
    //  b2,
    //  0.01,
    //  Texture::new_constant(scalar(0.5)),
    //)),
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
*/
