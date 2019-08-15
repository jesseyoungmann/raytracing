use crate::vec3::*;

pub struct ONB {
  pub u: Vec3,
  pub v: Vec3,
  pub w: Vec3,
}

impl ONB {
  pub fn build_from_w(n: Vec3) -> Self {
    let w = n.unit();
    let a = if w.x.abs() > 0.9 {
      vec3(0.0, 1.0, 0.0)
    } else {
      vec3(1.0, 0.0, 0.0)
    };
    let v = w.cross(a).unit();
    let u = w.cross(v);

    Self {
      u, v, w,
    }
  }

  pub fn local(&self, a: Vec3) -> Vec3 {
    scalar(a.x) * self.u
      + scalar(a.y) * self.v
      + scalar(a.z) * self.w
  }
}

// TODO: Indexable?
