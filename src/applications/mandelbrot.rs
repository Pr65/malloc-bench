#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]
use std::sync::Arc;
use std::sync::atomic::*;
use num::Complex;
use structopt::*;
use lazy_static::*;
include!("../global.rs");

#[cfg(not(test))]
set_alloc!();

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long)]
    thread: usize,
    #[structopt(short, long)]
    width: usize,
    #[structopt(short, long)]
    height: usize,
}

#[derive(Copy, Clone)]
struct Job(i32, i32);

struct ThreadData {
    buf: Vec<Vec<AtomicBool>>,
}

unsafe impl Sync for ThreadData {}

struct ThreadPool {
    data: Arc<ThreadData>,
    handles: Vec<std::thread::JoinHandle<()>>,
    width: usize,
    height: usize,
    thread: usize,
}

impl ThreadPool {
    fn new(width: usize, height: usize, thread: usize) -> Self {
        let mut buf = Vec::new();
        for _ in 0..width {
            let mut inner = Vec::new();
            for _ in 0..height {
                inner.push(AtomicBool::new(false));
            }
            buf.push(inner);
        }
        ThreadPool {
            data: Arc::new(ThreadData {
                buf
            }),
            handles: Vec::new(),
            width,
            height,
            thread
        }
    }
    fn execute(&mut self) {
        let mut i = 0;
        let total_count = self.height * self.width;
        let mut start = 0;
        while i < self.thread {
            let length =
                if (total_count - i) % self.thread > 0
                { (total_count - i) / self.thread + 1 } else { (total_count - i) / self.thread };
            let data = self.data.clone();
            let w = self.width;
            let h = self.height;
            self.handles.push(std::thread::spawn(move || {
                for j in start..start + length {
                    draw_step(data.buf.as_ref(), j % w, j / w, w, h);
                }
            }));
            i += 1;
            start += length;
        }
    }

    fn join(&mut self) {
        while !self.handles.is_empty() {
            let h = self.handles.pop();
            h.unwrap().join().unwrap();
        }
    }
}

fn draw_step(buf: &Vec<Vec<AtomicBool>>, i: usize, j: usize, width: usize, height: usize) {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new((i as f64 - (width as f64 / 2.0)) / (width as f64 / 4.0),
                         (j as f64 - (height as f64 / 2.0)) / (height as f64 / 4.0));
    let mut k = 0;
    loop {
        let temp = z.re * z.re - z.im * z.im + c.re;
        z.im = 2.0 * z.re * z.im + c.im;
        k += 1;
        z.re = temp;
        if z.norm_sqr() >= 4.0 || k >= 200 {
            break;
        }
    }
    if k >= 200 {
        buf[i][j].store(true, Ordering::Relaxed);
    }
}


fn main() {
    let opt : Opt = Opt::from_args();
    let start = std::time::Instant::now();
    let mut pool = ThreadPool::new(opt.width, opt.height, opt.thread);
    pool.execute();
    pool.join();
    let end = std::time::Instant::now();
    println!("bench: mandelbrot");
    println!("malloc: {}", crate::G_TYPE);
    println!("thread: {}", opt.thread);
    println!("width: {}", opt.width);
    println!("height: {}", opt.height);
    println!("ns: {}", end.duration_since(start).as_nanos())
}