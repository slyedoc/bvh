#![allow(warnings)]

mod ray;
use ray::*;

mod tri;
use tri::*;

mod bvh;
use bvh::*;


use std::{mem::swap, time::Instant};

use glam::*;
use image::{Luma, Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::*;

const HEIGHT: u32 = 640;
const WIDTH: u32 = 640;
const USE_BVH: bool = true;
const N: usize = 64;

fn main() {
    println!("size of N: {}", N);
    println!("using bvh: {}", USE_BVH);
    println!("size of BVHNode: {}", std::mem::size_of::<BvhNode>());

    let mut bvh = Bvh::new();
    gen_random_triangles(&mut bvh);
    bvh.setup();

    // render them out
    let camPos = Vec3::new(0.0, 0.0, -18.0);

    let p0 = vec3(-1.0, 1.0, -15.0);
    let p1 = vec3(1.0, 1.0, -15.0);
    let p2 = vec3(-1.0, -1.0, -15.0);

    let mut ray = Ray::default();

    let mut img = RgbImage::new(WIDTH, HEIGHT);
    let progress_bar = ProgressBar::new(HEIGHT as u64);

    let start = Instant::now();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixelPos = p0 + (p1 - p0) * (x as f32 / 640.0) + (p2 - p0) * (y as f32 / 640.0);
            ray.orgin = camPos;
            ray.distance = (pixelPos - ray.orgin).normalize();
            ray.t = 1e30;

            if USE_BVH {
                bvh.intersect(&mut ray, 0);
            } else {
                for i in 0..N {
                    ray.intersect_triangle(&bvh.triangles[i]);
                }
            }
            

            let color = if ray.t < 1e30 {
                [u8::MAX, u8::MAX, u8::MAX]
            } else {
                [0, 0, 0]
            };

            img.put_pixel(x, y, Rgb(color));
        }
        progress_bar.inc(1);
    }
    img.save("out.png").unwrap();
    progress_bar.finish_with_message("Complete!");

    let end = start.elapsed();
    println!("Time: {:?}", end);
}

fn gen_random_triangles(bvh: &mut Bvh) {
    // generate random triangles
    for i in 0..N {
        let mut rng = thread_rng();
        let r0 = vec3(
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
        );
        let r1 = vec3(
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
        );
        let r2 = vec3(
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
        );

        let v0 = r0 * 9.0 - Vec3::splat(5.0);
        bvh.add_triangle(Tri::new(v0, v0 + r1, v0 + r2));
    }
}
