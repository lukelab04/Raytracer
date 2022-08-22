use std::{sync::{Arc, mpsc}, thread::{self, JoinHandle}};

use rgraphics::{*, textures::*};
use rand::{SeedableRng, rngs::StdRng, Rng};
mod math;
mod object;
mod camera;
mod material;
use math::*;
use object::*;
use camera::*;
use material::*;

fn ray_color(r: &Ray, objects: &HittableObjects, depth: i32) -> Vec3 {
    if depth <= 0 {return Vec3::new(0.0, 0.0, 0.0)}
    let norm = r.direction.normalized();
    
    let mut hit = Hit::new();
    if objects.hit(&r, 0.00001, f64::INFINITY, &mut hit) {
        let mut scattered = Ray::new(Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,0.0));
        let mut attenuation = Vec3::new(0.0,0.0,0.0);

        if hit.material.as_ref().unwrap().scatter(&r, &hit, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, objects, depth - 1);
        } return Vec3::new(0.0,0.0,0.0);
    }

    let t = (norm.y() + 1.0) * 0.5;
    t * Vec3::new(0.1, 0.3, 1.0) + (1.0 - t) * Vec3::new(1.0, 1.0, 1.0)
}


fn gen_objs(objects: &mut HittableObjects) {

    let mut rnd = StdRng::from_entropy();

    let ground_mat = Lambertian::new(Vec3::new(0.5,0.5,0.5));
    objects.add(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(ground_mat)));

    const objs: i32 = 3;

    for a in -objs..objs {
        for b in -objs..objs {
            let rnd_mat = rnd.gen_range(0.0..1.0);
            let center = Vec3::new(a as f64 + rnd.gen_range(0.0..0.9), 0.2, b as f64 + rnd.gen_range(0.0..0.9));

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if rnd_mat < 0.8 {
                    let albedo = Vec3::rand_vec() * Vec3::rand_vec() * 0.6;
                    let mat = Lambertian::new(albedo);
                    objects.add(Sphere::new(center, 0.2, Arc::new(mat)));
                } else if rnd_mat < 0.9 {
                    let albedo = Vec3::rand_vec();
                    let fuzz = rnd.gen_range(0.0..0.6);
                    let mat = Metal::new(albedo, fuzz);
                    objects.add(Sphere::new(center, 0.2, Arc::new(mat)));
                } else {
                    let mat = Dielectric::new(1.5);
                    objects.add(Sphere::new(center, 0.2, Arc::new(mat)));
                }
            }
        }
    }

    let m1 = Dielectric::new(1.5);
    let m2 = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
    let m3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);

    objects.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Arc::new(m1)));
    objects.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(m2)));
    objects.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Arc::new(m3)));
}

fn main() {
    let (program, mut ev) = rgraphics::Program::new();

    let start = std::time::Instant::now();

    const AR: f64 = 9.0 / 16.0;
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = (WIDTH as f64 * AR) as u32;
    const SAMPLES: u32 = 500;
    const DEPTH: i32 = 100;

    let mut objects = HittableObjects::new();
    gen_objs(&mut objects);
    let objects = Arc::new(objects);

    let camera = Arc::new(
        Camera::new(AR, 1.0, Vec3::new(13.0, 2.0, -3.0), Vec3::new(0.0,0.0,0.0))
    );
    let mut tex = RenderTexture2D::new(&program.renderer, WIDTH, HEIGHT);

    let mut color_array = vec![Vec3::new(0.0,0.0,0.0); (WIDTH * HEIGHT) as usize];

    for j in 0..HEIGHT {
        println!("{}%", j as f64 / HEIGHT as f64 * 100.0);
        for i in 0..WIDTH {
            let mut handles: Vec<JoinHandle<()>> = vec![];
            for _ in 0..SAMPLES {
                let (tx, rx) = mpsc::channel();
                let thread_cam = camera.clone();
                let thread_objs = objects.clone();
                
                let rnd = StdRng::from_entropy();

                let handle = thread::spawn(move || {
                    let u = (i as f64 + rnd.clone().gen_range(0.0..0.6)) / (WIDTH - 1) as f64;
                    let v = (j as f64 + rnd.clone().gen_range(0.0..0.6)) / (HEIGHT - 1) as f64;
                    tx.send((i, j, ray_color(&thread_cam.as_ref().get_ray(u, v), &thread_objs.as_ref(), DEPTH))).unwrap();
                });
                handles.push(handle);

                for recieved in rx {
                    color_array[(recieved.1 * WIDTH + recieved.0) as usize] += recieved.2;
                }
            }
            for handle in handles {handle.join().unwrap()}
        }
    }

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let mut color = color_array[(j * WIDTH + i) as usize];
            let s = 1.0 / SAMPLES as f64;
            color[0] = f64::clamp(f64::sqrt(s * color.x()), 0.0, 0.999);
            color[1] = f64::clamp(f64::sqrt(s * color.y()), 0.0, 0.999);
            color[2] = f64::clamp(f64::sqrt(s * color.z()), 0.0, 0.999);
            tex.set_pixel(i, j, color.to_color());
        }
    }

    tex.apply(&program.renderer);

    println!("Time taken: {}", (std::time::Instant::now() - start).as_secs_f64());

    run(program, &mut ev, &mut |program| {
        program.draw_texture(-1.0, 1.0, 2.0, 2.0, &tex);
    });
}
