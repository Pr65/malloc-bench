#[cfg(test)]
mod vector_bench {
    use test::Bencher;
    use rand::{Rng, SeedableRng};
    use rand::prelude::{SliceRandom, StdRng};
    use rayon::prelude::*;

    #[bench]
    fn vector_push_100000(bencher: &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        let mut data = Vec::new();
        for _ in 0..100000 {
            data.push([rng.gen::<usize>(),
                rng.gen::<usize>(), rng.gen::<usize>(),
                rng.gen::<usize>(), rng.gen::<usize>(),
                rng.gen::<usize>(), rng.gen::<usize>()])
        }
        bencher.iter(|| {
            let mut new = Vec::new();
            for i in &data {
                new.push(i);
            }
        })
    }

    #[bench]
    fn vector_parallel_push_100000(bencher: &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        static mut DATA: Vec<[usize;7]> = Vec::new();
        for _ in 0..100000 {
            unsafe {
                DATA.push([rng.gen::<usize>(),
                    rng.gen::<usize>(), rng.gen::<usize>(),
                    rng.gen::<usize>(), rng.gen::<usize>(),
                    rng.gen::<usize>(), rng.gen::<usize>()]);
            }
        }
        bencher.iter(|| {
            let mut handles = Vec::new();
            for _ in 0..6 {
                handles.push(std::thread::spawn(|| {
                    let mut v = Vec::new();
                    unsafe {
                        for i in &DATA {
                            v.push(i);
                        }
                    }
                }));
            }
            for i in handles {
                i.join().unwrap();
            }
        })
    }

    #[bench]
    fn vector_rayon_map_collect_100000(bencher: &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        let mut data = Vec::new();
        for _ in 0..100000 {
            data.push(rng.gen::<usize>());
        }
        bencher.iter(|| {
            let vec = data
                .par_iter()
                .map(|x| x + 1)
                .collect::<Vec<_>>();
        })
    }

    #[bench]
    fn vector_rayon_map_flatten_collect_100000(bencher: &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        let mut data = Vec::new();
        for _ in 0..100000 {
            data.push(rng.gen::<usize>());
        }
        bencher.iter(|| {
            let vec = data
                .par_iter()
                .map(|x| {
                    vec![*x, x + 1, x + 2, x + 3, x + 4, x + 5, x + 6, x + 7, x + 8, x + 9]
                })
                .flatten()
                .collect::<Vec<_>>();
        })
    }

    #[bench]
    fn vector_map_flatten_collect_100000(bencher: &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        let mut data = Vec::new();
        for _ in 0..100000 {
            data.push(rng.gen::<usize>());
        }
        bencher.iter(|| {
            let vec = data
                .iter()
                .map(|x| {
                    vec![*x, x + 1, x + 2, x + 3, x + 4, x + 5, x + 6, x + 7, x + 8, x + 9]
                })
                .flatten()
                .collect::<Vec<_>>();
        })
    }


}
