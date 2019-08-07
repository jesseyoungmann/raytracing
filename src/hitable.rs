use crate::bvh::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::*;

#[derive(Default)]
pub struct HitRecord<'a> {
  pub t: f64,
  pub u: f64,
  pub v: f64,
  pub p: Vec3,
  pub normal: Vec3,
  pub material: Option<&'a Material>,
}

pub trait Hitable: std::fmt::Debug + Send + Sync {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
  fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb>;
}

#[derive(Debug)]
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
        let (u, v) = get_sphere_uv(rec.p);
        rec.u = u;
        rec.v = v;
        rec.normal = (rec.p - self.center) / scalar(self.radius);
        rec.material = Some(&self.material);
        return Some(rec);
      }

      temp = (-b + discriminant.sqrt()) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        let (u, v) = get_sphere_uv(rec.p);
        rec.u = u;
        rec.v = v;
        rec.normal = (rec.p - self.center) / scalar(self.radius);
        rec.material = Some(&self.material);
        return Some(rec);
      }
    }
    None
  }

  fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
    return Some(Aabb::new(
      self.center - scalar(self.radius),
      self.center + scalar(self.radius),
    ));
  }
}

#[derive(Debug)]
pub struct HitableList {
  pub list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
  pub fn new(list: Vec<Box<dyn Hitable>>) -> Self {
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

  fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
    if self.list.len() < 1 {
      return None;
    }
    let mut boxy = self.list[0].bounding_box(t0, t1)?;
    for i in 1..self.list.len() {
      boxy = surrounding_box(boxy, self.list[i].bounding_box(t0, t1)?);
    }
    Some(boxy)
  }
}

fn get_sphere_uv(p: Vec3) -> (f64, f64) {
  let phi = p.z.atan2(p.x);
  let theta = p.y.asin();
  let pi = std::f64::consts::PI;
  let u = 1.0 - (phi + pi) / (2.0 * pi);
  let v = (theta + pi / 2.0) / pi;
  (u, v)
}

#[derive(Debug, Clone)]
pub struct XYRect {
  pub material: Material,
  pub x0: f64,
  pub x1: f64,
  pub y0: f64,
  pub y1: f64,
  pub k: f64,
}

impl XYRect {
  pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Material) -> Self {
    Self {
      x0,
      x1,
      y0,
      y1,
      k,
      material,
    }
  }
}

impl Hitable for XYRect {
  fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
    let t = (self.k - r.origin().z) / r.direction().z;
    if t < t0 || t > t1 {
      return None;
    }
    let x = r.origin().x + t * r.direction().x;
    let y = r.origin().y + t * r.direction().y;
    if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
      return None;
    }

    Some(HitRecord {
      u: (x - self.x0) / (self.x1 - self.x0),
      v: (y - self.y0) / (self.y1 - self.y0),
      t: t,
      material: Some(&self.material),
      p: r.point_at_parameter(t),
      normal: vec3(0.0, 0.0, 1.0),
    })
  }

  fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
    Some(Aabb::new(
      vec3(self.x0, self.y0, self.k - 0.0001),
      vec3(self.x1, self.y1, self.k + 0.0001),
    ))
  }
}

#[derive(Debug, Clone)]
pub struct XZRect {
  pub material: Material,
  pub x0: f64,
  pub x1: f64,
  pub z0: f64,
  pub z1: f64,
  pub k: f64,
}

impl XZRect {
  pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
    Self {
      x0,
      x1,
      z0,
      z1,
      k,
      material,
    }
  }
}

impl Hitable for XZRect {
  fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
    let t = (self.k - r.origin().y) / r.direction().y;
    if t < t0 || t > t1 {
      return None;
    }
    let x = r.origin().x + t * r.direction().x;
    let z = r.origin().z + t * r.direction().z;
    if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
      return None;
    }

    Some(HitRecord {
      u: (x - self.x0) / (self.x1 - self.x0),
      v: (z - self.z0) / (self.z1 - self.z0),
      t: t,
      material: Some(&self.material),
      p: r.point_at_parameter(t),
      normal: vec3(0.0, 1.0, 0.0),
    })
  }

  fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
    Some(Aabb::new(
      vec3(self.x0, self.k - 0.0001, self.z0),
      vec3(self.x1, self.k + 0.0001, self.z1),
    ))
  }
}

#[derive(Debug, Clone)]
pub struct YZRect {
  pub material: Material,
  pub y0: f64,
  pub y1: f64,
  pub z0: f64,
  pub z1: f64,
  pub k: f64,
}

impl YZRect {
  pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
    Self {
      y0,
      y1,
      z0,
      z1,
      k,
      material,
    }
  }
}

impl Hitable for YZRect {
  fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
    let t = (self.k - r.origin().x) / r.direction().x;
    if t < t0 || t > t1 {
      return None;
    }
    let y = r.origin().y + t * r.direction().y;
    let z = r.origin().z + t * r.direction().z;
    if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
      return None;
    }

    Some(HitRecord {
      u: (y - self.y0) / (self.y1 - self.y0),
      v: (z - self.z0) / (self.z1 - self.z0),
      t: t,
      material: Some(&self.material),
      p: r.point_at_parameter(t),
      normal: vec3(1.0, 0.0, 0.0),
    })
  }

  fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
    Some(Aabb::new(
      vec3(self.k - 0.0001, self.y0, self.z0),
      vec3(self.k + 0.0001, self.y1, self.z1),
    ))
  }
}

#[derive(Debug, Clone)]
pub enum FlipNormals {
  OkayXY(XYRect),
  OkayXZ(XZRect),
  OkayYZ(YZRect),
}

use FlipNormals::*;

impl FlipNormals {
  pub fn new_xy(hitable: XYRect) -> Self {
    OkayXY(hitable)
  }
  pub fn new_xz(hitable: XZRect) -> Self {
    OkayXZ(hitable)
  }
  pub fn new_yz(hitable: YZRect) -> Self {
    OkayYZ(hitable)
  }
}

impl Hitable for FlipNormals {
  fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
    let rec = match &self {
      OkayXY(hitable) => hitable.hit(r, t0, t1),
      OkayXZ(hitable) => hitable.hit(r, t0, t1),
      OkayYZ(hitable) => hitable.hit(r, t0, t1),
    };
    if let Some(mut rec) = rec {
      rec.normal = -rec.normal;
      return Some(rec);
    }
    None
  }
  fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
    match &self {
      OkayXY(hitable) => hitable.bounding_box(t0, t1),
      OkayXZ(hitable) => hitable.bounding_box(t0, t1),
      OkayYZ(hitable) => hitable.bounding_box(t0, t1),
    }
  }
}
