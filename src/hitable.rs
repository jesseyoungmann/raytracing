use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::*;

#[derive(Default)]
pub struct HitRecord<'a> {
  pub t: f64,
  pub p: Vec3,
  pub normal: Vec3,
  pub material: Option<&'a Material>,
}

pub trait Hitable {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
  pub center: Vec3,
  pub radius: f64,
  pub material: Material,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
    Self {
      center,
      radius,
      material,
    }
  }
}

impl Hitable for Sphere {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let oc = r.origin() - self.center;
    let a = r.direction().dot(r.direction());
    let b = oc.dot(r.direction());
    let c = oc.dot(oc) - self.radius * self.radius;
    let discriminant = b * b - a * c;
    if discriminant > 0.0 {
      let mut rec = HitRecord::default();

      let mut temp = (-b - discriminant.sqrt()) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p - self.center) / scalar(self.radius);
        rec.material = Some(&self.material);
        return Some(rec);
      }

      temp = (-b + discriminant.sqrt()) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p - self.center) / scalar(self.radius);
        rec.material = Some(&self.material);
        return Some(rec);
      }
    }
    None
  }
}

pub struct HitableList {
  pub list: Vec<Sphere>,
}

impl HitableList {
  pub fn new(list: Vec<Sphere>) -> Self {
    Self { list }
  }
}

impl Hitable for HitableList {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut hit_anything = None;
    let mut closest_so_far = t_max;

    for hitable in &self.list {
      if let Some(rec) = hitable.hit(r, t_min, closest_so_far) {
        closest_so_far = rec.t;
        hit_anything = Some(rec);
      }
    }
    return hit_anything;
  }
}
