mod ray;
use ray::*;
mod tri;
use tri::*;
mod bvh;
pub use bvh::*;
mod camera;
use camera::*;

use glam::*;
use image::{Rgb, RgbImage};
use rand::*;
use rand_chacha::ChaCha8Rng;

pub fn run(
    rng: &mut ChaCha8Rng,
    width: u32,
    height: u32,
    count: usize,
    no_bvh: bool,
    filename: Option<String>,
) {
    // Camera
    let camera = Camera::new(
        width,
        height,
        vec3(0.0, 0.0, -18.0),
        vec3(0.0, 0.0, 0.0),
        Vec3::Y,
        45.0,
    );

    // Generate something to see, seeding random for debugging

    let tris = gen_random_triangles(count, rng);

    let mut bvh = Bvh::new(count);
    let mut img = RgbImage::new(width, height);

    if !no_bvh {
        //let bvh_setup = Instant::now();
        bvh.setup(&tris);
        //println!("Bvh Setup: {:?}", bvh_setup.elapsed());
    }

    //let start = Instant::now();
    let mut ray = Ray {
        origin: camera.position,
        ..Default::default()
    };


    for y in 0..height {
        for x in 0..width {
            //let u = x as f32 / width as f32;
            //let v = y as f32 / height as f32;
            ray.direction =
                camera.lower_left_corner + (x as f32 * camera.x_step) + (y as f32 * camera.y_step)
                    - camera.position;

            // ray.direction = camera.lower_left_corner + (u as f32 * camera.horizontal) + (v as f32 * camera.vertical)
            // - camera.position;
            ray.t = 1e30;

            if no_bvh {
                // brute force
                for t in &tris {
                    ray.intersect_triangle(t);
                }
            } else {
                //ray.intersect_bvh(bvh, 0, &tris);
                bvh.intersect(&mut ray, 0, &tris);
            }

            let color = if ray.t < 1e30 {
                [u8::MAX, u8::MAX, u8::MAX]
            } else {
                [0, 0, 0]
            };

            img.put_pixel(x, y, Rgb(color));
        }
    }
    //let time = start.elapsed();
    //println!("Render Time: {:?}", time);

    // println!(
    //     "Ray per second: {:0.1}M",
    //     (width as f32 * height as f32) / time.as_secs_f32() / 1_000_000f32
    // );

    // Save the image
    if let Some(filename) = filename {
        img.save(filename).unwrap();
    }    
}

// generate random triangles
fn gen_random_triangles(size: usize, rng: &mut ChaCha8Rng) -> Vec<Tri> {
    (0..size)
        .map(|_| {
            let r0 = random_vec3(rng);
            let r1 = random_vec3(rng);
            let r2 = random_vec3(rng);

            let v0 = r0 * 5.0;
            Tri::new(v0, v0 + r1, v0 + r2)
        })
        .collect::<Vec<_>>()
}

fn random_vec3(rng: &mut ChaCha8Rng) -> Vec3 {
    vec3(
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
    )
}


