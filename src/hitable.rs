use rand::prelude::*;

use crate::bvh::*;
use crate::material::*;
use crate::ray::Ray;
use crate::texture::Texture;
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

#[derive(Debug)]
pub struct Cuboid {
  pmin: Vec3,
  pmax: Vec3,
  list: HitableList,
}

impl Hitable for Cuboid {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    self.list.hit(r, t_min, t_max)
  }
  fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
    Some(Aabb::new(self.pmin, self.pmax))
  }
}

impl Cuboid {
  pub fn new(p0: Vec3, p1: Vec3, material: Material) -> Self {
    let list: Vec<Box<dyn Hitable>> = vec![
      Box::new(XYRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, material.clone())),
      Box::new(FlipNormals::new_xy(XYRect::new(
        p0.x,
        p1.x,
        p0.y,
        p1.y,
        p0.z,
        material.clone(),
      ))),
      Box::new(XZRect::new(
        p0.x,
        p1.x,
        p0.z,
        p1.z,
        p1.y + 0.01,
        material.clone(),
      )),
      Box::new(FlipNormals::new_xz(XZRect::new(
        p0.x,
        p1.x,
        p0.z,
        p1.z,
        p0.y,
        material.clone(),
      ))),
      Box::new(YZRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, material.clone())),
      Box::new(FlipNormals::new_yz(YZRect::new(
        p0.y,
        p1.y,
        p0.z,
        p1.z,
        p0.x,
        material.clone(),
      ))),
    ];

    Self {
      pmin: p0,
      pmax: p1,
      list: HitableList::new(list),
    }
  }
}

#[derive(Debug)]
pub struct Translate {
  hitable: Box<dyn Hitable>,
  offset: Vec3,
}

impl Translate {
  pub fn new(hitable: Box<dyn Hitable>, offset: Vec3) -> Self {
    Self { hitable, offset }
  }
}

impl Hitable for Translate {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let moved_r = Ray::new(r.origin() - self.offset, r.direction());
    if let Some(mut rec) = self.hitable.hit(&moved_r, t_min, t_max) {
      rec.p += self.offset;
      return Some(rec);
    }
    None
  }
  fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
    if let Some(boxy) = self.hitable.bounding_box(t0, t1) {
      return Some(Aabb::new(boxy.min + self.offset, boxy.max + self.offset));
    }
    None
  }
}

#[derive(Debug)]
pub struct RotateY {
  hitable: Box<dyn Hitable>,
  sin_theta: f64,
  cos_theta: f64,
  hasbox: bool,
  boxy: Aabb,
}

impl RotateY {
  pub fn new(hitable: Box<dyn Hitable>, angle: f64) -> Self {
    let pi = std::f64::consts::PI;
    let radians = (pi / 180.0) * angle;
    let sin_theta = radians.sin();
    let cos_theta = radians.cos();
    let boxy = hitable.bounding_box(0.0, 1.0);
    let hasbox = boxy.is_some();
    let mut min = scalar(std::f64::MAX);
    let mut max = scalar(std::f64::MIN);

    //WHAT?
    let bbox = boxy.expect("DOUBLE CHECK WHAT HAPPENS HERE"); //unwrap_or(Aabb::new(scalar(0.0),scalar(0.0)));
    for i in 0..2 {
      for j in 0..2 {
        for k in 0..2 {
          let x = i as f64 * bbox.max.x + (1.0 - i as f64) * bbox.min.x;
          let y = j as f64 * bbox.max.y + (1.0 - j as f64) * bbox.min.y;
          let z = k as f64 * bbox.max.z + (1.0 - k as f64) * bbox.min.z;

          let newx = cos_theta * x + sin_theta * z;
          let newz = -sin_theta * x + cos_theta * z;
          let tester = vec3(newx, y, newz);
          for c in 0..3 {
            if tester[c] > max[c] {
              max[c] = tester[c];
            }
            if tester[c] < min[c] {
              min[c] = tester[c];
            }
          }
        }
      }
    }
    Self {
      hitable,
      sin_theta,
      cos_theta,
      hasbox,
      boxy: Aabb::new(min, max),
    }
  }
}

impl Hitable for RotateY {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut origin = r.origin();
    let mut direction = r.direction();
    let cos_theta = self.cos_theta;
    let sin_theta = self.sin_theta;
    origin.x = cos_theta * r.origin().x - sin_theta * r.origin().z;
    origin.z = sin_theta * r.origin().x + cos_theta * r.origin().z;
    direction.x = cos_theta * r.direction().x - sin_theta * r.direction().z;
    direction.z = sin_theta * r.direction().x + cos_theta * r.direction().z;
    let rotated_r = Ray::new(origin, direction);

    if let Some(mut rec) = self.hitable.hit(&rotated_r, t_min, t_max) {
      let mut p = rec.p;
      let mut normal = rec.normal;
      p.x = cos_theta * rec.p.x + sin_theta * rec.p.z;
      p.z = -sin_theta * rec.p.x + cos_theta * rec.p.z;
      normal.x = cos_theta * rec.normal.x + sin_theta * rec.normal.z;
      normal.z = -sin_theta * rec.normal.x + cos_theta * rec.normal.z;
      rec.p = p;
      rec.normal = normal;
      return Some(rec);
    }
    None
  }
  fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
    if self.hasbox {
      Some(self.boxy)
    } else {
      None
    }
  }
}

#[derive(Debug)]
pub struct ConstantMedium {
  pub boundary: Box<dyn Hitable>,
  pub density: f64,
  pub phase_function: Material,
}

impl ConstantMedium {
  pub fn new(boundary: Box<dyn Hitable>, density: f64, a: Texture) -> Self {
    Self {
      boundary,
      density,
      phase_function: Isotropic::new(a),
    }
  }
}

impl Hitable for ConstantMedium {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut rng = rand::thread_rng();
    //let mut db = rng.gen::<f64>() < 0.00001;
    //db = false;
    if let Some(mut rec1) = self.boundary.hit(r, std::f64::MIN, std::f64::MAX) {
      if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, std::f64::MAX) {
        if rec1.t < t_min {
          rec1.t = t_min;
        }
        if rec2.t > t_max {
          rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
          return None;
        }
        if rec1.t < 0.0 {
          rec1.t = 0.0;
        }

        let distance_inside_boundary = (rec2.t - rec1.t) * r.direction().length();
        // TODO: CHECK C++ LOG FUNCTION BASE
        let hit_distance = -(1.0 / self.density) * rng.gen::<f64>().ln();

        if hit_distance < distance_inside_boundary {
          let t = rec1.t + hit_distance / r.direction().length();
          return Some(HitRecord {
            t: t,
            u: 0.0,
            v: 0.0,
            p: r.point_at_parameter(t),
            normal: vec3(1.0, 0.0, 0.0), // arbitrary
            material: Some(&self.phase_function),
          });
        }
      }
    }
    None
  }

  fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
    self.boundary.bounding_box(t0, t1)
  }
}
