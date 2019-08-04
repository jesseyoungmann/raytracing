use rand::prelude::*;
use std::cmp::Ordering;

use crate::hitable::*;
use crate::ray::*;
use crate::vec3::*;

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
  pub min: Vec3,
  pub max: Vec3,
}

impl Aabb {
  pub fn new(min: Vec3, max: Vec3) -> Self {
    Self { min, max }
  }

  #[allow(dead_code)]
  pub fn slower_hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
    for a in 0..3 {
      let direction = r.direction()[a];
      let min_a = self.min[a];
      let max_a = self.max[a];
      let origin_a = r.origin()[a];

      let t0 = ffmin(
        (min_a - origin_a) / direction,
        (max_a - origin_a) / direction,
      );

      let t1 = ffmax(
        (min_a - origin_a) / direction,
        (max_a - origin_a) / direction,
      );

      t_min = ffmax(t0, t_min);
      t_max = ffmin(t1, t_max);
      if t_max <= t_min {
        return false;
      }
    }
    true
  }

  pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
    for a in 0..3 {
      let inv_d = 1.0 / r.direction()[a];
      let mut t0 = (self.min[a] - r.origin()[a]) * inv_d;
      let mut t1 = (self.max[a] - r.origin()[a]) * inv_d;
      if inv_d < 0.0 {
        std::mem::swap(&mut t0, &mut t1);
      }
      t_min = if t0 > t_min { t0 } else { t_min };
      t_max = if t1 < t_max { t1 } else { t_max };
      if t_max <= t_min {
        return false;
      }
    }
    true
  }
}

fn ffmin(a: f64, b: f64) -> f64 {
  if a < b {
    a
  } else {
    b
  }
}

fn ffmax(a: f64, b: f64) -> f64 {
  if a > b {
    a
  } else {
    b
  }
}

pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
  let small = vec3(
    ffmin(box0.min.x, box1.min.x),
    ffmin(box0.min.y, box1.min.y),
    ffmin(box0.min.z, box1.min.z),
  );

  let big = vec3(
    ffmax(box0.max.x, box1.max.x),
    ffmax(box0.max.y, box1.max.y),
    ffmax(box0.max.z, box1.max.z),
  );

  return Aabb::new(small, big);
}

#[derive(Debug)]
pub struct BvhNode<'a> {
  // Okay, this should either own it, or contain a specific item?
  pub contents: Option<&'a dyn Hitable>,
  pub left: Option<Box<BvhNode<'a>>>,
  pub right: Option<Box<BvhNode<'a>>>,
  pub boxy: Aabb,
}

impl<'a> BvhNode<'a> {
  pub fn new(l: &'a mut [&dyn Hitable], time0: f64, time1: f64) -> Self {
    let axis = (3.0 * rand::thread_rng().gen::<f64>()) as isize;
    l.sort_unstable_by(if axis == 0 {
      box_x_compare
    } else if axis == 1 {
      box_y_compare
    } else {
      box_z_compare
    });

    // should be possible to avoid cloning and just box the remains of the vec in place
    if l.len() == 1 {
      Self {
        contents: Some(&*l[0]),
        left: None,
        right: None,
        boxy: l[0]
          .bounding_box(time0, time1)
          .expect("No bounding box in BvhNode::new() 1"),
      }
    } else {
      let (left_vec, right_vec) = l.split_at_mut(l.len() / 2);
      let left = Box::new(BvhNode::new(left_vec, time0, time1));
      let right = Box::new(BvhNode::new(right_vec, time0, time1));

      let box_left = left.bounding_box(time0, time1);
      let box_right = right.bounding_box(time0, time1);

      assert!(
        box_left.is_some() && box_right.is_some(),
        "No bounding box in BvhNode::new() 2"
      );

      Self {
        contents: None,
        left: Some(left),
        right: Some(right),
        boxy: surrounding_box(box_left.unwrap(), box_right.unwrap()),
      }
    }
  }
}

fn box_x_compare(a: &&dyn Hitable, b: &&dyn Hitable) -> Ordering {
  let box_left = a.bounding_box(0.0, 0.0);
  let box_right = b.bounding_box(0.0, 0.0);

  assert!(
    box_left.is_some() && box_right.is_some(),
    "No bounding box in BvhNode::new()"
  );

  if box_left.unwrap().min.x - box_right.unwrap().min.x < 0.0 {
    Ordering::Less
  } else {
    Ordering::Greater
  }
}

fn box_y_compare(a: &&dyn Hitable, b: &&dyn Hitable) -> Ordering {
  let box_left = a.bounding_box(0.0, 0.0);
  let box_right = b.bounding_box(0.0, 0.0);

  assert!(
    box_left.is_some() && box_right.is_some(),
    "No bounding box in BvhNode::new()"
  );

  if box_left.unwrap().min.y - box_right.unwrap().min.y < 0.0 {
    Ordering::Less
  } else {
    Ordering::Greater
  }
}

fn box_z_compare(a: &&dyn Hitable, b: &&dyn Hitable) -> Ordering {
  let box_left = a.bounding_box(0.0, 0.0);
  let box_right = b.bounding_box(0.0, 0.0);

  assert!(
    box_left.is_some() && box_right.is_some(),
    "No bounding box in BvhNode::new()"
  );

  if box_left.unwrap().min.z - box_right.unwrap().min.z < 0.0 {
    Ordering::Less
  } else {
    Ordering::Greater
  }
}

impl Hitable for BvhNode<'_> {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    return if self.boxy.hit(r, t_min, t_max) {
      if let Some(hit_content) = self.contents.and_then(|h| h.hit(r, t_min, t_max)) {
        return Some(hit_content);
      }

      let hit_left: Option<HitRecord> = self
        .left
        .as_ref()
        .and_then(|h: &Box<BvhNode>| h.hit(r, t_min, t_max));
      let hit_right: Option<HitRecord> = self
        .right
        .as_ref()
        .and_then(|h: &Box<BvhNode>| h.hit(r, t_min, t_max));
      if hit_left.is_some() && hit_right.is_some() {
        if hit_left.as_ref().unwrap().t < hit_right.as_ref().unwrap().t {
          hit_left
        } else {
          hit_right
        }
      } else if hit_left.is_some() {
        hit_left
      } else if hit_right.is_some() {
        hit_right
      } else {
        None
      }
    } else {
      None
    };
  }

  fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
    Some(self.boxy)
  }
}
