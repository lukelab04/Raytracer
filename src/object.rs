use std::{rc::Rc, sync::Arc};

use crate::{math::*, material::*};


pub struct HittableObjects {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableObjects {
    pub fn new() -> HittableObjects {
        HittableObjects { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, o: impl Hittable + 'static) {
        self.objects.push(Box::new(o));
    }

    pub fn hit(&self, r: &Ray, min: f64, max: f64, rec: &mut Hit) -> bool {
        let mut tmphit = Hit::new();
        let mut hit_anything = false;
        let mut closest = max;

        for elem in &self.objects {
            if elem.hit(&r, min, max, &mut tmphit) {
                hit_anything = true;
                if tmphit.t < closest {
                    closest = tmphit.t;
                    *rec = tmphit.clone();
                }
            }
        }

        hit_anything
    }
}

unsafe impl Send for HittableObjects {
    
}

unsafe impl Sync for HittableObjects {}

#[derive(Clone)]
pub struct Hit {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Arc<dyn Material>>,
}

impl Hit {
    pub fn set_face_normal(&mut self, r: &Ray, outward: &Vec3) {
        self.front_face = r.direction.dot(&outward) < 0.0;
        self.normal = if self.front_face {*outward} else {-*outward}
    }

    pub fn new() -> Hit {
        Hit { p: Point::new(0.0, 0.0, 0.0), normal: Point::new(0.0, 0.0, 0.0), t: 0.0, 
            front_face: true, material: None }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hit) -> bool;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere {center, radius, material}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Hit) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let disc = half_b * half_b - a * c;
        if disc < 0.0 {return false;}
        let sqrtd = f64::sqrt(disc);


        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root { return false; }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        rec.material = Some(self.material.clone());

        let outward = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward);

        true

    }
}

