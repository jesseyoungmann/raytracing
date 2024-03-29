use std::fmt;
use std::ops::{
  Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
  Vec3 { x, y, z }
}
pub fn scalar(f: f64) -> Vec3 {
  vec3(f, f, f)
}

impl Vec3 {
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Self { x, y, z }
  }

  pub fn normalize(&self) -> Self {
    let total = (self.x + self.y + self.z).abs();
    Self {
      x: self.x / total,
      y: self.y / total,
      z: self.z / total,
    }
  }

  pub fn r(&self) -> f64 {
    self.x
  }
  pub fn g(&self) -> f64 {
    self.y
  }
  pub fn b(&self) -> f64 {
    self.z
  }
}

impl fmt::Display for Vec3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {} {}", self.x, self.y, self.z)
  }
}

// Regular

impl Add for Vec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Sub for Vec3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Mul for Vec3 {
  type Output = Self;

  fn mul(self, other: Self) -> Self {
    Self {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z,
    }
  }
}

impl Div for Vec3 {
  type Output = Self;

  fn div(self, other: Self) -> Self {
    Self {
      x: self.x / other.x,
      y: self.y / other.y,
      z: self.z / other.z,
    }
  }
}

// Unary

impl Neg for Vec3 {
  type Output = Self;

  fn neg(self) -> Self {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

// Assignment

impl AddAssign for Vec3 {
  fn add_assign(&mut self, other: Self) {
    *self = *self + other;
  }
}

impl SubAssign for Vec3 {
  fn sub_assign(&mut self, other: Self) {
    *self = *self - other;
  }
}

impl MulAssign for Vec3 {
  fn mul_assign(&mut self, other: Self) {
    *self = *self * other;
  }
}

impl DivAssign for Vec3 {
  fn div_assign(&mut self, other: Self) {
    *self = *self / other;
  }
}

impl Index<usize> for Vec3 {
  type Output = f64;

  fn index(&self, index: usize) -> &Self::Output {
    match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => panic!("Invalid index into Vec3"),
    }
  }
}

impl IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    match index {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      _ => panic!("Invalid index into Vec3"),
    }
  }
}

// Fancy math

impl Vec3 {
  pub fn length(&self) -> f64 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }

  pub fn squared_length(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn dot(&self, other: Self) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  pub fn cross(&self, other: Self) -> Self {
    Self::new(
      self.y * other.z - self.z * other.y,
      -(self.x * other.z - self.z * other.x),
      self.x * other.y - self.y * other.x,
    )
  }

  pub fn unit(&self) -> Self {
    *self * scalar(1.0 / self.length())
  }
}
