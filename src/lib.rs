#![feature(weak_into_raw)]
#![feature(test)]
#![feature(allocator_api)]
mod data_structures;

#[cfg(feature = "bench_rpmalloc")]
#[global_allocator]
static GLOBAL: rpmalloc::RpMalloc = rpmalloc::RpMalloc;

extern crate test;
#[cfg(any(feature = "bench_mimalloc", feature = "bench_smimalloc", feature = "bench_fsmimalloc"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "bench_jemalloc")]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(feature = "bench_wee_alloc")]
#[global_allocator]
static GLOBAL: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "bench_tcmalloc")]
#[global_allocator]
static GLOBAL: tcmalloc::TCMalloc = tcmalloc::TCMalloc;

#[cfg(any(feature = "bench_snmalloc", feature = "bench_snmalloc-1mib"))]
#[global_allocator]
static GLOBAL: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

fn main() {
    println!("Hello, world!");
}
