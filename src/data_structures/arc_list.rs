#![allow(dead_code)]
use std::sync::Arc;
use std::fmt::{Debug, Formatter, Error};
use crate::data_structures::arc_list::ArcList::*;
use test::Bencher;
enum ArcList<T> {
    Cons(T, Arc<Self>),
    Nil
}

impl<T> ArcList<T> {
    fn is_nil(&self) -> bool {
        match self {
            Nil => true,
            _ => false
        }
    }
}

impl<T : Debug> Debug for ArcList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Nil => Ok(()),
            Cons(e, t) if t.is_nil() => write!(f, "{:?}", e),
            Cons(e, t) => write!(f, "{:?}, {:?}", e, *(*t))
        }
    }
}
type Ptr<T> = Arc<ArcList<T>>;

fn cons<T>(t: T, list: Ptr<T>) -> Ptr<T> {
    Arc::new(Cons(t, list.clone()))
}

fn count_inner<T>(acc: usize, list: Ptr<T>) -> usize {
    match & *list {
        Nil => acc,
        Cons(_, t) => count_inner(acc + 1, t.clone())
    }
}

fn count<T>(list: Ptr<T>) -> usize {
    count_inner(0, list)
}

fn to_vec_inner<T : Clone>(mut acc: Vec<T>, list: Ptr<T>) -> Vec<T> {
    match & *list {
        Nil => acc,
        Cons(x, t) => {
            acc.push(x.clone());
            to_vec_inner(acc, t.clone())
        }
    }
}

fn to_vec<T : Clone>(list: Ptr<T>) -> Vec<T> {
    to_vec_inner(Vec::new(), list)
}

fn map<T, U>(f: fn(&T) -> U, list: Ptr<T>) -> Ptr<U> {
    match &* list {
        Nil => Arc::new(Nil),
        Cons(x, t) => Arc::new(Cons(f(x), map(f, t.clone())))
    }
}


#[cfg(test)]
mod list_bench {
    use std::sync::Arc;
    use super::*;
    use rand::{Rng, SeedableRng};
    use rand::prelude::StdRng;

    const SCALE: usize = 10000;
    #[bench]
    fn long_cons_then_count(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
            let mut a = Arc::new(Nil);
            for _ in 0..SCALE {
                a = cons(rng.gen::<usize>(), a);
            }
            assert_eq!(count(a), SCALE)
        });
    }

    #[bench]
    fn long_cons_then_map(bencher: &mut Bencher) {
        bencher.iter(|| {
            let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
            let mut a = Arc::new(Nil);
            for _ in 0..SCALE {
                a = cons(rng.gen::<usize>(), a);
            }
            map(|x| x + 1, a);
        });
    }

    #[bench]
    fn long_cons_then_count_in_multi_threads(bencher: &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        static mut DATA : Vec<usize> = Vec::new();
        for _ in 0..SCALE {
            unsafe {
                DATA.push(rng.gen::<usize>());
            }
        }
        bencher.iter(move || {
            let mut handles = Vec::new();
            for _ in 0..6 {
                handles.push(std::thread::Builder::new()
                    .stack_size(512 * 1024 * 1024)
                    .spawn(move || {
                        let mut a = Arc::new(Nil);
                        unsafe {
                            for i in &DATA {
                                a = cons(i, a);
                            }
                        }
                    assert_eq!(count(a), SCALE)
                }).unwrap());
            }
            for i in handles {i.join().unwrap(); }
        });
    }

    #[bench]
    fn long_cons_then_map_across_multi_threads(bencher: &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(*crate::SEED);
        static mut DATA : Vec<usize> = Vec::new();
        for _ in 0..SCALE {
            unsafe {
                DATA.push(rng.gen::<usize>());
            }
        }
        bencher.iter( || {
            let mut handles = Vec::new();
            let mut a = Arc::new(Nil);
            unsafe {
                for i in &DATA {
                    a = cons(*i, a);
                }
            }
            for _ in 0..6 {
                let a = a.clone();
                handles.push(std::thread::Builder::new()
                    .stack_size(512 * 1024 * 1024)
                    .spawn(|| {
                        map(|x| x + 1, a);
                    }).unwrap());
            }
            for i in handles {i.join().unwrap(); }
        });
    }
}
