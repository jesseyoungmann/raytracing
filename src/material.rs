use crate::hitable::*;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vec3::*;
use rand::prelude::*;

pub enum Material {
  OkayLambertian(Lambertian),
  OkayMetal(Metal),
  OkayDielectric(Dielectric),
}

impl Material {
  pub fn scatter(&self, r_in: &Ray, rec: &mut HitRecord) -> Option<(Vec3, Ray)> {
    match self {
      Material::OkayLambertian(inner) => inner.scatter(r_in,rec),
      Material::OkayMetal(inner) => inner.scatter(r_in,rec),
      Material::OkayDielectric(inner) => inner.scatter(r_in,rec),
    }
  }
}


#[derive(Clone)]
pub struct Lambertian {
  pub albedo: Vec3,
}

impl Lambertian {
  pub fn new(albedo: Vec3) -> Material {
    Material::OkayLambertian(Self { albedo })
  }

  pub fn scatter(&self, _r_in: &Ray, rec: &mut HitRecord) -> Option<(Vec3, Ray)> {
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
  pub fn new(albedo: Vec3, fuzz: f64) -> Material {
    Material::OkayMetal(Self {
      albedo,
      fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
    })
  }

  pub fn scatter(&self, r_in: &Ray, rec: &mut HitRecord) -> Option<(Vec3, Ray)> {
    let reflected = reflect(r_in.direction().unit(), rec.normal);
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

#[derive(Clone)]
pub struct Dielectric {
  pub ref_idx: f64,
}

impl Dielectric {
  pub fn new(ref_idx: f64) -> Material {
    Material::OkayDielectric(Self { ref_idx })
  }

  pub fn scatter(&self, r_in: &Ray, rec: &mut HitRecord) -> Option<(Vec3, Ray)> {
    let reflected = reflect(r_in.direction(), rec.normal);
    let attenuation = vec3(1.0, 1.0, 1.0);

    let outward_normal: Vec3;
    let ni_over_nt: f64;
    let cosine;
    if r_in.direction().dot(rec.normal) > 0.0 {
      outward_normal = -rec.normal;
      ni_over_nt = self.ref_idx;
      cosine = self.ref_idx * r_in.direction().dot(rec.normal) / r_in.direction().length();
    } else {
      outward_normal = rec.normal;
      ni_over_nt = 1.0 / self.ref_idx;
      cosine = -r_in.direction().dot(rec.normal) / r_in.direction().length();
    }

    let reflect_prob;
    let refracted;
    if let Some(r) = refract(r_in.direction(), outward_normal, ni_over_nt) {
      refracted = Some(r);
      reflect_prob = schlick(cosine, self.ref_idx);
    } else {
      refracted = None;
      reflect_prob = 1.0;
    }

    let scattered = if rand::thread_rng().gen::<f64>() < reflect_prob {
      Ray::new(rec.p, reflected)
    } else {
      Ray::new(rec.p, refracted.expect("WTF, NO REFRACTED"))
    };

    Some((attenuation, scattered))
  }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
  v - scalar(2.0) * scalar(v.dot(n)) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
  let uv = v.unit();
  let dt = uv.dot(n);
  let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
  if discriminant > 0.0 {
    return Some(scalar(ni_over_nt) * (uv - n * scalar(dt)) - n * scalar(discriminant.sqrt()));
  }
  None
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
  let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
  let r0 = r0 * r0;
  r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
