use rand::{thread_rng, Rng};

use crate::{math::*, object::*};


pub trait Material {
    fn scatter(&self, in_ray: &Ray, hit: &Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {Lambertian { albedo }}
}

impl Material for Lambertian {
    fn scatter(&self, in_ray: &Ray, hit: &Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut dir = hit.normal + Vec3::rand_in_unit_sphere().normalized();

        if dir.near_zero() {dir = hit.normal}

        *scattered = Ray::new(hit.p, dir);
        *attenuation = self.albedo;
        true
    }

}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {Metal { albedo, fuzz }}
}

impl Material for Metal {
    fn scatter(&self, in_ray: &Ray, hit: &Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = in_ray.direction.normalized().reflect(&hit.normal);
        *scattered = Ray::new(hit.p, reflected + self.fuzz * Vec3::rand_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction.dot(&hit.normal) > 0.0
    }

}


pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {Dielectric { ir }}
    pub fn reflectence(&self, cos: f64, refidx: f64) -> f64 {
        let r0 = (1.0 - refidx) / (1.0 + refidx);
        let r1 = r0 * r0;
        r1 + (1.0 - r1) * f64::powi(1.0 - cos, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, in_ray: &Ray, hit: &Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut rnd = thread_rng();
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refract = if hit.front_face {1.0 / self.ir} else {self.ir};
        let u_dir = in_ray.direction.normalized();

        let ct = f64::min((-u_dir).dot(&hit.normal), 1.0);
        let st = f64::sqrt(1.0 - ct * ct);
        let no_refract = refract * st > 1.0;
        let dir = if no_refract || self.reflectence(ct, refract) > rnd.gen_range(0.0..1.0) {u_dir.reflect(&hit.normal)} 
            else {u_dir.refract(&hit.normal, refract)};

        *scattered = Ray::new(hit.p, dir);
        true
    }
}