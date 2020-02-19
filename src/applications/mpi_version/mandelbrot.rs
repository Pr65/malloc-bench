#![allow(dead_code)]
#![allow(unused_imports)]
use mpi::traits::*;
use structopt::*;
use num::Complex;
include!("../../global.rs");

const ROOT: usize = 0;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long)]
    width: usize,
    #[structopt(short, long)]
    height: usize,
}

fn validate(w: usize, h: usize, i: usize, j: usize) -> bool {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new((i as f64 - (w as f64 / 2.0)) / (w as f64 / 4.0),
                         (j as f64 - (h as f64 / 2.0)) / (h as f64 / 4.0));
    let mut k = 0;
    loop {
        let temp = z.re * z.re - z.im * z.im + c.re;
        z.im = 2.0 * z.re * z.im + c.im;
        k += 1;
        z.re = temp;
        if z.norm_sqr() >= 4.0 || k >= 100 {
            break;
        }
    }
    k >= 100
}

fn main() {
    let univ = mpi::initialize().unwrap();
    let world = univ.world();
    let size = world.size() as usize;
    let rank = world.rank() as usize;
    let root = world.process_at_rank(ROOT as i32);
    let mut width = 0;
    let mut height = 0;
    if rank == ROOT {
        let opt: Opt = Opt::from_args();
        width = opt.width;
        height = opt.height;
        println!("bench: mandelbrot");
        println!("malloc: {}", crate::G_TYPE);
        println!("process: {}", size);
        println!("width: {}", opt.width);
        println!("height: {}", opt.height);
    }
    let start_time = std::time::Instant::now();
    root.broadcast_into(&mut width);
    root.broadcast_into(&mut height);
    let mut vec_start: Vec<usize> = Vec::new();
    let mut vec_length: Vec<usize> = Vec::new();
    let total_size = height * width;
    let block_size = if total_size % size > 0 { total_size / size + 1 } else { total_size / size };
    if rank == ROOT {
        let mut start = 0;
        for i in 0..size {
            vec_start.push(start);
            vec_length.push(if (total_size - i) % size > 0
            { (total_size - i) / size + 1 } else { (total_size - i) / size });
            start += vec_length[i];
        }
    }
    let mut start = 0;
    let mut length = 0;

    if rank == ROOT {
        root.scatter_into_root(&vec_start[..], &mut start);
        root.scatter_into_root(&vec_length[..], &mut length);
    } else {
        root.scatter_into(&mut start);
        root.scatter_into(&mut length);
    }

    let mut data = Vec::new();
    for i in 0..length {
        data.push(validate(width, height, (start + i) % (width), (start + i) / (width)));
    }

    if rank >= (height * width) % size && (height * width) % size > 0 {
        data.push(false);
    }

    let mut recv = Vec::new();
    if rank == ROOT {
        recv.resize((block_size * size) as usize, false);
        root.gather_into_root(&data[..], &mut recv[..]);
        let end = std::time::Instant::now();
        println!("ns: {}", end.duration_since(start_time).as_nanos())
    } else {
        root.gather_into(&data[..]);
    }
}