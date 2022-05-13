#![allow(warnings)]

mod ray;
use rand_chacha::ChaCha8Rng;
use ray::*;

mod tri;
use tri::*;

mod bvh;
use bvh::*;

use std::{env, mem::swap, time::Instant};

use clap::*;

use glam::*;
use image::{Luma, Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::*;
/// Optional args
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 64)]
    count: usize,
    #[clap(short, long, default_value_t = 512)]
    width: u32,
    #[clap(short, long, default_value_t = 640)]
    height: u32,
    #[clap(short, long)]
    no_bvh: bool,
    #[clap(short, long, default_value_t = 1)]
    seed: u64,
    #[clap(short, long, default_value = "out.png")]
    output: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    println!("BVHNode mem size: {} bytes", std::mem::size_of::<BvhNode>());

    // render them out
    let camera_position = Vec3::new(0.0, 0.0, -18.0);
    let p0 = vec3(-1.0, 1.0, -15.0);
    let p1 = vec3(1.0, 1.0, -15.0);
    let p2 = vec3(-1.0, -1.0, -15.0);

    let mut ray = Ray {
        origin: camera_position,
        ..Default::default()
    };

    // Generate something to see
    let mut rng = ChaCha8Rng::seed_from_u64(args.seed);
    let tris = gen_random_triangles(args.count, &mut rng);

    let mut bvh = Bvh::new(args.count);

    if !args.no_bvh {
        let bvh_setup = Instant::now();
        bvh.setup(&tris);
        println!("Bvh Setup: {:?}", bvh_setup.elapsed());
    }

    let mut img = RgbImage::new(args.width, args.height);
    let start = Instant::now();
    for y in 0..args.height {
        for x in 0..args.width {
            let pixelPos = p0
                + (p1 - p0) * (x as f32 / args.width as f32)
                + (p2 - p0) * (y as f32 / args.height as f32);

            ray.origin = camera_position;
            ray.direction = (pixelPos - ray.origin).normalize();

            ray.t = 1e30;

            if args.no_bvh {
                // brute force
                for i in 0..tris.len() {
                    ray.intersect_triangle(&tris[i]);
                }
            } else {
                // bvh
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

    let time = start.elapsed();
    println!("Render Time: {:?}, {}", time, time.as_secs_f32());
    println!(
        "Ray per second: {:0.1}M/s",
        (args.width as f32 * args.height as f32) / time.as_secs_f32() / 1_000_000f32
    );

    // Save the image
    img.save(args.output).unwrap();
}

// generate random triangles
fn gen_random_triangles(size: usize, rng: &mut ChaCha8Rng) -> Vec<Tri> {
    (0..size)
        .map(|_| {
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
            Tri::new(v0, v0 + r1, v0 + r2)
        })
        .collect::<Vec<_>>()
}
