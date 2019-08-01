use crate::ray::Ray;
use crate::vec3::*;
use rand::prelude::*;

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  pub u: Vec3,
  pub v: Vec3,
  pub w: Vec3,
  pub lens_radius: f64,
}

impl Camera {
  pub fn new(
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
    vfov: f64,
    aspect: f64,
    aperture: f64,
    focus_dist: f64,
  ) -> Self {
    let lens_radius = aperture / 2.0;
    // vfov is top to bottom in degrees
    let theta = vfov * std::f64::consts::PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;

    let origin = lookfrom;

    let w = (lookfrom - lookat).unit();
    let u = vup.cross(w).unit();
    let v = w.cross(u);

    Self {
      lower_left_corner: origin
        - scalar(half_width * focus_dist) * u
        - scalar(half_height * focus_dist) * v
        - scalar(focus_dist) * w,
      horizontal: scalar(2.0) * scalar(half_width * focus_dist) * u,
      vertical: scalar(2.0) * scalar(half_height * focus_dist) * v,
      origin: origin,
      u: u,
      v: v,
      w: w,
      lens_radius: lens_radius,
    }
  }

  pub fn get_ray(&self, s: f64, t: f64) -> Ray {
    let rd = scalar(self.lens_radius) * random_in_unit_disk();
    let offset = self.u * scalar(rd.x) + self.v * scalar(rd.y);
    Ray::new(
      self.origin + offset,
      self.lower_left_corner + scalar(s) * self.horizontal + scalar(t) * self.vertical
        - self.origin
        - offset,
    )
  }
}

fn random_in_unit_disk() -> Vec3 {
  let mut p;
  let mut rng = rand::thread_rng();
  loop {
    p = scalar(2.0) * vec3(rng.gen(), rng.gen(), 0.0) - vec3(1.0, 1.0, 0.0);
    if p.dot(p) < 1.0 {
      break;
    }
  }
  p
}
