use crate::ray::Ray;
use crate::vec3::*;

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

impl Camera {
  pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Self {
    // vfov is top to bottom in degrees
    let theta = vfov * std::f64::consts::PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;

    let origin = lookfrom;

    let w = (lookfrom - lookat).unit();
    let u = vup.cross(w).unit();
    let v = w.cross(u);

    Self {
      lower_left_corner: origin - scalar(half_width) * u - scalar(half_height) * v - w,
      horizontal: scalar(2.0) * scalar(half_width) * u,
      vertical: scalar(2.0) * scalar(half_height) * v,
      origin: origin,
    }
  }

  pub fn get_ray(&self, s: f64, t: f64) -> Ray {
    Ray::new(
      self.origin,
      self.lower_left_corner + scalar(s) * self.horizontal + scalar(t) * self.vertical
        - self.origin,
    )
  }
}
