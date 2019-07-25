use crate::hitable::*;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vec3::*;

pub trait Material {
  fn scatter(&self, r_in: &Ray, rec: &mut HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Clone)]
pub struct Lambertian {
  pub albedo: Vec3,
}

impl Material for Lambertian {
  fn scatter(&self, _r_in: &Ray, rec: &mut HitRecord) -> Option<(Vec3, Ray)> {
    let target = rec.p + rec.normal + random_in_unit_sphere();
    let scattered = Ray::new(rec.p, target - rec.p);
    let attenuation = self.albedo;
    Some((attenuation, scattered))
  }
}

#[derive(Clone)]
pub struct Metal {
  pub albedo: Vec3,
  pub fuzz: f64,
}

impl Metal {
  pub fn new(albedo: Vec3, fuzz: f64) -> Self {
    Self {
      albedo,
      fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
    }
  }

  fn reflect(&self, v: Vec3, n: Vec3) -> Vec3 {
    v - scalar(2.0) * scalar(v.dot(n)) * n
  }
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &mut HitRecord) -> Option<(Vec3, Ray)> {
    let reflected = self.reflect(r_in.direction().unit(), rec.normal);
    let scattered = Ray::new(
      rec.p,
      reflected + scalar(self.fuzz) * random_in_unit_sphere(),
    );
    let attenuation = self.albedo;
    if scattered.direction().dot(rec.normal) > 0.0 {
      Some((attenuation, scattered))
    } else {
      None
    }
  }
}
