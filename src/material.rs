use rand::prelude::*;

use crate::hitable::*;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::texture::*;
use crate::vec3::*;

#[derive(Debug, Clone)]
pub enum Material {
  OkayLambertian(Lambertian),
  OkayMetal(Metal),
  OkayDielectric(Dielectric),
  OkayDiffuseLight(DiffuseLight),
}

use Material::*;
impl Material {
  pub fn scatter(&self, r_in: &Ray, rec: &mut HitRecord) -> Option<(Vec3, Ray)> {
    match self {
      OkayLambertian(inner) => inner.scatter(r_in, rec),
      OkayMetal(inner) => inner.scatter(r_in, rec),
      OkayDielectric(inner) => inner.scatter(r_in, rec),
      OkayDiffuseLight(inner) => inner.scatter(r_in, rec),
    }
  }

  pub fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
    match self {
      OkayDiffuseLight(inner) => inner.emitted(u, v, p),
      _ => scalar(0.0),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Lambertian {
  pub albedo: Texture,
}

impl Lambertian {
  pub fn new(texture: Texture) -> Material {
    OkayLambertian(Self { albedo: texture })
  }

  pub fn new_from_color(albedo: Vec3) -> Material {
    OkayLambertian(Self {
      albedo: Texture::new_constant(albedo),
    })
  }

  pub fn scatter(&self, _r_in: &Ray, rec: &mut HitRecord) -> Option<(Vec3, Ray)> {
    let target = rec.p + rec.normal + random_in_unit_sphere();
    let scattered = Ray::new(rec.p, target - rec.p);
    let attenuation = self.albedo.value(0.0, 0.0, rec.p);
    Some((attenuation, scattered))
  }
}

#[derive(Clone, Debug)]
pub struct Metal {
  pub albedo: Vec3,
  pub fuzz: f64,
}

impl Metal {
  pub fn new(albedo: Vec3, fuzz: f64) -> Material {
    OkayMetal(Self {
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

#[derive(Clone, Debug)]
pub struct Dielectric {
  pub ref_idx: f64,
}

impl Dielectric {
  pub fn new(ref_idx: f64) -> Material {
    OkayDielectric(Self { ref_idx })
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

#[derive(Debug, Clone)]
pub struct DiffuseLight {
  emit: Texture,
}

impl DiffuseLight {
  pub fn new(emit: Texture) -> Material {
    OkayDiffuseLight(Self { emit })
  }

  pub fn scatter(&self, _r_in: &Ray, _rec: &mut HitRecord) -> Option<(Vec3, Ray)> {
    None
  }

  pub fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
    self.emit.value(u, v, p)
  }
}
