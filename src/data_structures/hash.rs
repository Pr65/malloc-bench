#[cfg(test)]
mod btree_bench {
    use test::Bencher;
    use rand::prelude::StdRng;
    use rand::{SeedableRng, Rng};
    use hashbrown::HashSet;
    use rayon::prelude::*;
    #[bench]
    fn set_insert_100000(bc : &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        let mut data = Vec::new();
        for _ in 0..100000 {
            data.push(rng.gen::<usize>());
        }
        bc.iter(|| {
            let mut set = HashSet::new();
            for i in &data {
                set.insert(i);
            }
        })
    }

    #[bench]
    fn set_query_1000000(bc : &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        let mut data = Vec::new();
        let mut set = HashSet::new();
        for _ in 0..1000000 {
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
    fn set_rayon_query_1000000 (bc : &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        let mut data = Vec::new();
        let mut set = HashSet::new();
        for _ in 0..1000000 {
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