use crate::ray::Ray;
use crate::vec3::*;

#[derive(Default, Clone)]
pub struct HitRecord {
  pub t: f64,
  pub p: Vec3,
  pub normal: Vec3,
}

pub trait Hitable {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
  pub center: Vec3,
  pub radius: f64,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f64) -> Self {
    Self { center, radius }
  }
}

impl Hitable for Sphere {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    let oc = r.origin() - self.center;
    let a = r.direction().dot(r.direction());
    let b = oc.dot(r.direction());
    let c = oc.dot(oc) - self.radius * self.radius;
    let discriminant = b * b - a * c;
    if discriminant > 0.0 {
      let mut temp = (-b - discriminant.sqrt()) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p - self.center) / scalar(self.radius);
        return true;
      }

      temp = (-b + discriminant.sqrt()) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p - self.center) / scalar(self.radius);
        return true;
      }
    }
    false
  }
}

pub struct HitableList {
  pub list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
  pub fn new(list: Vec<Box<dyn Hitable>>) -> Self {
    Self { list }
  }
}

impl Hitable for HitableList {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    let mut temp_rec = HitRecord::default();
    let mut hit_anything = false;
    let mut closest_so_far = t_max;

    for hitable in &self.list {
      if hitable.hit(r, t_min, closest_so_far, &mut temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        *rec = temp_rec.clone();
      }
    }
    return hit_anything;
  }
}
