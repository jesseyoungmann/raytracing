use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub enum Texture {
  ConstantTexture {
    color: Vec3,
  },
  CheckerTexture {
    odd: Box<Texture>,
    even: Box<Texture>,
  },
}

use Texture::*;

impl Texture {
  pub fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
    match self {
      ConstantTexture { color } => *color,
      CheckerTexture { odd, even } => {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 { odd } else { even }.value(u, v, p)
      }
    }
  }

  pub fn new_constant(color: Vec3) -> Self {
    ConstantTexture { color }
  }

  pub fn new_checker(odd: Self, even: Self) -> Self {
    CheckerTexture {
      odd: Box::new(odd),
      even: Box::new(even),
    }
  }
}
