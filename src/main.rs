#![allow(warnings)]
use bvh_tutorial::*;
use clap::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value_t = 64)]
    pub count: usize,
    #[clap(short, long, default_value_t = 640)]
    pub width: u32,
    #[clap(short, long, default_value_t = 640)]
    pub height: u32,
    #[clap(short, long)]
    pub no_bvh: bool,
    #[clap(short, long, default_value_t = 1)]
    pub seed: u64,
    #[clap(short, long, default_value = "out.png")]
    pub output: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    println!("BVHNode mem size: {} bytes", std::mem::size_of::<BvhNode>());
    let mut rng = ChaCha8Rng::seed_from_u64(args.seed);

    bvh_tutorial::run(
        &mut rng,
        args.width,
        args.height,
        args.count,
        args.no_bvh,
        Some(args.output),
    );
}
