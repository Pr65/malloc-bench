#[allow(unused_macros)]
macro_rules! set_alloc {
() => {
        #[cfg(feature = "bench_rpmalloc")]
        #[global_allocator]
        static GLOBAL: rpmalloc::RpMalloc = rpmalloc::RpMalloc;

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

        #[cfg(any(feature = "bench_snmalloc", feature = "bench_snmalloc-1mib", feature = "bench_snmalloc-cache"))]
        #[global_allocator]
        static GLOBAL: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

    }
}

pub static SEED : [u8; 32] = [
     1, 2, 3, 4, 5, 6, 7, 8,
     1, 2, 3, 4, 5, 6, 7, 8,
     1, 2, 3, 4, 5, 6, 7, 8,
     1, 2, 3, 4, 5, 6, 7, 8,
];

#[cfg(feature = "bench_rpmalloc")]
pub static G_TYPE : &'static str = "rpmalloc";

#[cfg(any(feature = "bench_mimalloc", feature = "bench_smimalloc", feature = "bench_fsmimalloc"))]
pub static G_TYPE : &'static str = "mimalloc";

#[cfg(feature = "bench_jemalloc")]
pub static G_TYPE : &'static str = "jemalloc";

#[cfg(feature = "bench_wee_alloc")]
pub static G_TYPE : &'static str = "wee_alloc";

#[cfg(feature = "bench_tcmalloc")]
pub static G_TYPE : &'static str = "tcmalloc";

#[cfg(any(feature = "bench_snmalloc", feature = "bench_snmalloc-1mib", feature = "bench_snmalloc-cache"))]
pub static G_TYPE : &'static str = "snmalloc";

#[cfg(not(any(
     feature = "bench_snmalloc",
     feature = "bench_snmalloc-1mib",
     feature = "bench_snmalloc-cache",
     feature = "bench_tcmalloc",
     feature = "bench_wee_alloc",
     feature = "bench_jemalloc",
     feature = "bench_mimalloc",
     feature = "bench_smimalloc",
     feature = "bench_fsmimalloc",
     feature = "bench_rpmalloc"
)))]
pub static G_TYPE : &'static str = "std-malloc";


