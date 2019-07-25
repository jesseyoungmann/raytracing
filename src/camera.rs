use crate::ray::Ray;
use crate::vec3::*;

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

impl Camera {
  pub fn new(origin: Vec3, lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Self {
    Self {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
    }
  }

  pub fn specific() -> Self {
    Self::new(
      scalar(0.0),
      vec3(-2.0, -1.0, -1.0),
      vec3(4.0, 0.0, 0.0),
      vec3(0.0, 2.0, 0.0),
    )
  }

  pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    Ray::new(
      self.origin,
      self.lower_left_corner + scalar(u) * self.horizontal + scalar(v) * self.vertical
        - self.origin,
    )
  }
}
