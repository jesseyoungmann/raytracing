use rand::prelude::*;

use crate::vec3::*;

#[derive(Clone)]
pub struct Perlin {
  ranvec: [Vec3; 256],
  perm_x: [usize; 256],
  perm_y: [usize; 256],
  perm_z: [usize; 256],
}

impl std::fmt::Debug for Perlin {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Perlin {{}}")
  }
}

impl Perlin {
  pub fn new() -> Self {
    Self {
      ranvec: perlin_generate(),
      perm_x: perlin_generate_perm(),
      perm_y: perlin_generate_perm(),
      perm_z: perlin_generate_perm(),
    }
  }

  pub fn noise(&self, p: Vec3) -> f64 {
    let u = p.x - p.x.floor();
    let v = p.y - p.y.floor();
    let w = p.z - p.z.floor();

    let i = p.x.floor() as usize;
    let j = p.y.floor() as usize;
    let k = p.z.floor() as usize;

    let mut c = [[[scalar(0.0); 2]; 2]; 2];

    for di in 0..2 {
      for dj in 0..2 {
        for dk in 0..2 {
          c[di][dj][dk] = self.ranvec[self.perm_x[(i + di) & 255]
            ^ self.perm_y[(j + dj) & 255]
            ^ self.perm_z[(k + dk) & 255]];
        }
      }
    }
    perlin_interpolate(&c, u, v, w)
  }

  pub fn turb(&self, p: Vec3, depth: usize) -> f64 {
    let mut accum = 0.0;
    let mut temp_p = p;
    let mut weight = 1.0;

    for _ in 0..depth {
      accum += weight * self.noise(temp_p);
      weight *= 0.5;
      temp_p *= scalar(2.0);
    }
    accum.abs()
  }
}

// LAZY CONST? DO JUST ONCE?
fn perlin_generate() -> [Vec3; 256] {
  let mut rng = rand::thread_rng();
  let mut p = [scalar(0.0); 256];
  for i in 0..256 {
    p[i] = vec3(
      -1.0 + 2.0 * rng.gen::<f64>(),
      -1.0 + 2.0 * rng.gen::<f64>(),
      -1.0 + 2.0 * rng.gen::<f64>(),
    )
    .unit()
  }
  p
}

fn permute(p: &mut [usize; 256]) {
  let n = p.len();
  let mut rng = rand::thread_rng();
  for i in 0..n {
    let i = n - i - 1;
    let target = (rng.gen::<f64>() as usize) * (i + 1);
    let tmp = p[i];
    p[i] = p[target];
    p[target] = tmp;
  }
}

fn perlin_generate_perm() -> [usize; 256] {
  let mut p = [0; 256];
  for i in 0..256 {
    p[i] = i;
  }
  permute(&mut p);
  p
}

fn perlin_interpolate(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
  let uu = u * u * (3.0 - 2.0 * u);
  let vv = v * v * (3.0 - 2.0 * v);
  let ww = w * w * (3.0 - 2.0 * w);

  let mut accum = 0.0;
  for ii in 0..2 {
    let i = ii as f64;
    for jj in 0..2 {
      let j = jj as f64;
      for kk in 0..2 {
        let k = kk as f64;
        let weight_v = vec3(u - i, v - j, w - k);

        accum += (i * uu + (1.0 - i) * (1.0 - uu))
          * (j * vv + (1.0 - j) * (1.0 - vv))
          * (k * ww + (1.0 - k) * (1.0 - ww))
          * c[ii][jj][kk].dot(weight_v)
      }
    }
  }
  accum
}
