#![allow(warnings)]

mod ray;
use ray::*;
mod tri;
use tri::*;
mod bvh;
use bvh::*;

mod camera;
use camera::*;

use std::{
    default,
    env::{self, args},
    mem::swap,
    time::Instant,
};

use clap::*;

use glam::*;
use image::{Luma, Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::*;
use rand_chacha::ChaCha8Rng;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 5)]
    count: usize,
    #[clap(short, long, default_value_t = 640)]
    width: u32,
    #[clap(short, long, default_value_t = 640)]
    height: u32,
    #[clap(short, long)]
    no_bvh: bool,
    #[clap(short, long, default_value_t = 5)]
    seed: u64,
    #[clap(short, long, default_value = "out.png")]
    output: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    println!("BVHNode mem size: {} bytes", std::mem::size_of::<BvhNode>());

    // Camera
    let camera = Camera::new(
        vec3(0.0, 0.0, -18.0),
        vec3(0.0, 0.0, 0.0),
        Vec3::Y,
        30.0,
        args.width as f32 / args.height as f32,
    );

    // Generate something to see
    let mut rng = ChaCha8Rng::seed_from_u64(args.seed);
    let tris = gen_random_triangles(args.count, &mut rng);

    let mut bvh = Bvh::new(args.count);

    let mut img = RgbImage::new(args.width, args.height);

    let bvh_option = if !args.no_bvh {
        let bvh_setup = Instant::now();
        bvh.setup(&tris);
        println!("Bvh Setup: {:?}", bvh_setup.elapsed());
        Some(&bvh)
    } else {
        None
    };

    let start = Instant::now();
    render_image(args.width, args.height, camera, tris, bvh_option, &mut img);
    let time = start.elapsed();
    println!("Render Time: {:?}, {}", time, time.as_secs_f32());

    println!(
        "Ray per second: {:0.1}M/s",
        (args.width as f32 * args.height as f32) / time.as_secs_f32() / 1_000_000f32
    );

    // Save the image
    img.save(args.output).unwrap();
}

fn render_image(
    width: u32,
    height: u32,
    camera: Camera,
    tris: Vec<Tri>,
    bvh_option: Option<&Bvh>,
    img: &mut image::ImageBuffer<Rgb<u8>, Vec<u8>>,
) {
    let mut ray = Ray {
        origin: camera.position,
        ..Default::default()
    };

    // let projection_matrix = camera.get_projection_matrix();
    // println!("{:?}", projection_matrix);

    for y in 0..height {
        for x in 0..width {
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;

            ray.direction = camera.get_direction(u, v);
            ray.t = 1e30;

            if let Some(bvh) = bvh_option {
                ray.intersect_bvh(bvh, 0, &tris);
            } else {
                // brute force
                for i in 0..tris.len() {
                    ray.intersect_triangle(&tris[i]);
                }
            }
            // if ray.t != 1e30 {
            //     print!("not black");
            // }

            let color = if ray.t < 1e30 {
                [u8::MAX, u8::MAX, u8::MAX]
            } else {
                [0, 0, 0]
            };

            img.put_pixel(x, y, Rgb(color));
        }
    }
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

            let v0 = r0 * 5.0;
            Tri::new(v0, v0 + r1, v0 + r2)
        })
        .collect::<Vec<_>>()
}
