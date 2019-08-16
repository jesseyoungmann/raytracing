use rand::prelude::*;

use crate::hitable::*;
use crate::onb::*;
use crate::random_cosine_direction;
use crate::vec3::*;

pub struct CosinePDF {
  uvw: ONB,
}

impl CosinePDF {
  pub fn new(w: Vec3) -> Self {
    Self {
      uvw: ONB::build_from_w(w),
    }
  }

  pub fn value(&self, direction: Vec3) -> f64 {
    let cosine = direction.unit().dot(self.uvw.w);
    if cosine > 0.0 {
      cosine / std::f64::consts::PI
    } else {
      0.0
    }
  }

  pub fn generate(&self) -> Vec3 {
    self.uvw.local(random_cosine_direction())
  }
}

pub struct HitablePDF<'a> {
  o: Vec3,
  hitable: &'a dyn Hitable,
}

impl<'a> HitablePDF<'a> {
  pub fn new(hitable: &'a dyn Hitable, o: Vec3) -> Self {
    Self { o, hitable }
  }

  pub fn value(&self, direction: Vec3) -> f64 {
    self.hitable.pdf_value(self.o, direction)
  }

  pub fn generate(&self) -> Vec3 {
    self.hitable.random(self.o)
  }
}

pub struct MixturePDF<'a> {
  pub hitable_pdf: HitablePDF<'a>,
  pub cosine_pdf: CosinePDF,
}

impl<'a> MixturePDF<'a> {
  pub fn new(hitable_pdf: HitablePDF<'a>, cosine_pdf: CosinePDF) -> Self {
    Self {
      hitable_pdf,
      cosine_pdf,
    }
  }

  pub fn value(&self, direction: Vec3) -> f64 {
    0.5 * self.hitable_pdf.value(direction) + 0.5 * self.cosine_pdf.value(direction)
  }

  pub fn generate(&self) -> Vec3 {
    if rand::thread_rng().gen::<bool>() {
      self.hitable_pdf.generate()
    } else {
      self.cosine_pdf.generate()
    }
  }
}
