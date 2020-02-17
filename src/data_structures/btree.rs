#[cfg(test)]
mod btree_bench {
    use test::Bencher;
    use rand::prelude::StdRng;
    use rand::{SeedableRng, Rng};
    use std::collections::BTreeSet;
    use rayon::prelude::*;

    #[bench]
    fn set_insert_10000(bc : &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        let mut data = Vec::new();
        for _ in 0..10000 {
            data.push(rng.gen::<usize>());
        }
        bc.iter(|| {
            let mut set = BTreeSet::new();
            for i in &data {
                set.insert(i);
            }
        })
    }

    #[bench]
    fn set_query_100000(bc : &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        let mut data = Vec::new();
        let mut set = BTreeSet::new();
        for _ in 0..100000 {
            let c = rng.gen::<usize>();
            set.insert(c);
            data.push(c);
        }
        bc.iter(|| {
            for i in &data {
                set.contains(i);
            }
        })
    }


    #[bench]
    fn set_rayon_query_100000 (bc : &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        let mut data = Vec::new();
        let mut set = BTreeSet::new();
        for _ in 0..100000 {
            let c = rng.gen::<usize>();
            set.insert(c);
            data.push(c);
        }
        bc.iter(|| {
            data.par_iter().for_each(|x|{
                set.contains(x);
            });
        })
    }
}