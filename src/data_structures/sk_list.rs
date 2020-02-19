#[cfg(test)]
mod sklist_bench {
    use test::Bencher;
    use rand::prelude::StdRng;
    use rand::Rng;
    use rayon::prelude::*;

    #[bench]
    fn skset_rayon_insert_10000(bc : &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(crate::SEED);
        static mut DATA : Vec<usize> = Vec::new();
        for _ in 0..10000 {
            unsafe {
                DATA.push(rng.gen::<usize>());
            }
        }
        bc.iter(|| {
            let set = crossbeam_skiplist::set::SkipSet::new();
            unsafe {
                DATA.par_iter().for_each(|x| {
                    set.insert(x);
                })
            }
        })
    }

    #[bench]
    fn skset_ser_insert_rayon_query_10000(bc : &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(crate::SEED);
        static mut SET : Option<crossbeam_skiplist::set::SkipSet<usize>> = None;
        unsafe {
            SET.replace(crossbeam_skiplist::set::SkipSet::new());

        }
        bc.iter(|| {
            let mut data = Vec::new();
            for _ in 0..10000 {
                unsafe {
                    let u = rng.gen::<usize>();
                    SET.as_ref().unwrap().insert(u);
                    data.push(u);
                }
            }

            unsafe {
                data.par_iter().for_each(|x| {
                    SET.as_ref().unwrap().contains(x);
                })
            }
        })
    }


}