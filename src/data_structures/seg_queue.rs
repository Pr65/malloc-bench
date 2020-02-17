#![allow(unused_must_use)]
#[cfg(test)]
mod seg_queue_bench {
    use test::Bencher;
    use rand::prelude::StdRng;
    use rand::{SeedableRng, Rng};
    use rayon::prelude::*;

    #[bench]
    fn rayon_insert_100000(bc : &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        let mut data = Vec::new();
        for _ in 0..100000 {
            data.push(rng.gen::<usize>());
        }
        bc.iter(|| {
            let queue = crossbeam::queue::SegQueue::new();
            data.par_iter().for_each(|x| {
                queue.push(x);
            })
        })
    }

    #[bench]
    fn ser_insert_par_pop_100000(bc : &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        static mut Q : Option<crossbeam::queue::SegQueue<usize>> = None;
        unsafe {
            Q.replace(crossbeam::queue::SegQueue::new());
        }
        bc.iter(|| {
            for _ in 0..100000 {
                unsafe {
                    Q.as_ref().unwrap().push(rng.gen::<usize>());
                }
            }
            let mut handles = Vec::new();
            for _ in 0..6 {
                unsafe {
                    handles.push(std::thread::spawn(|| {
                        while !Q.as_ref().unwrap().is_empty() {
                            Q.as_ref().unwrap().pop();
                        }
                    }));
                }
            }
            for i in handles {
                i.join().unwrap();
            }
        })
    }

}