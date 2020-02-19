#![allow(dead_code)]
use std::cell::RefCell;
use std::fmt::{Debug, Error, Formatter};
use std::rc::{Rc, Weak};

struct BinaryNode<T> {
    element: T,
    children: [Option<Rc<RefCell<Self>>>; 2],
    father: Option<Weak<RefCell<Self>>>,
}

type Ptr<T> = Rc<RefCell<BinaryNode<T>>>;
type WPtr<T> = Weak<RefCell<BinaryNode<T>>>;

fn new_cell<T>(t: T) -> Ptr<T> {
    Rc::new(RefCell::new(BinaryNode {
        element: t,
        children: [None, None],
        father: None,
    }))
}

fn new_cell_with_father<T>(t: T, father: Option<WPtr<T>>) -> Ptr<T> {
    Rc::new(RefCell::new(BinaryNode {
        element: t,
        children: [None, None],
        father,
    }))
}

fn insert<T: PartialOrd>(ptr: &mut Option<Ptr<T>>, t: T, father: Option<WPtr<T>>) -> Option<Ptr<T>> {
    if ptr.is_none() {
        ptr.replace(new_cell_with_father(t, father));
        return ptr.clone();
    } else {
        let inner = ptr.as_mut().unwrap();
        let mut node = inner.borrow_mut();
        if t < node.element {
            insert(&mut node.children[0], t, Some(Rc::downgrade(inner)))
        } else if t > node.element {
            insert(&mut node.children[1], t, Some(Rc::downgrade(inner)))
        } else {
            None
        }
    }
}

fn locate<T: PartialOrd>(ptr: &mut Option<Ptr<T>>, t: T) -> Option<Ptr<T>> {
    if ptr.is_none() {
        None
    } else {
        let inner = ptr.as_ref().cloned().unwrap();
        let mut node = inner.borrow_mut();
        if t == node.element {
            drop(node);
            Some(inner)
        } else if t < node.element {
            locate(&mut node.children[0], t)
        } else {
            locate(&mut node.children[1], t)
        }
    }
}

fn left_most<T>(mut ptr: Ptr<T>) -> Ptr<T> {
    let mut left = ptr.borrow().children[0].as_ref().cloned();
    while left.is_some() {
        ptr = left.unwrap();
        left = ptr.borrow().children[0].as_ref().cloned();
    }
    ptr
}

fn right_most<T>(mut ptr: Ptr<T>) -> Ptr<T> {
    let mut right = ptr.borrow().children[1].as_ref().cloned();
    while right.is_some() {
        ptr = right.unwrap();
        right = ptr.borrow().children[1].as_ref().cloned();
    }
    ptr
}

fn remove<T>(ptr: Ptr<T>, root: &mut Option<Ptr<T>>) -> Ptr<T> {
    let addr = ptr.as_ptr();
    let replacement = {
        let mut inner = ptr.borrow_mut();
        match &mut inner.children {
            [None, None] => None,
            [l, None] => l.take(),
            [None, r] => r.take(),
            [l, r] => {
                let y = r.take().unwrap();
                let x = l.take();
                drop(inner);
                let child = remove(left_most(y.clone()), &mut None);
                let mut node = child.borrow_mut();
                let y = ptr.borrow_mut().children[1].take();
                node.children = [x, y];
                node.children[0].as_mut().unwrap().borrow_mut().father = Some(Rc::downgrade(&child));
                if let Some(y) = node.children[1].as_mut() {
                    y.borrow_mut().father = Some(Rc::downgrade(&child));
                }
                drop(node);
                Some(child)
            }
        }
    };
    for i in replacement.as_ref().cloned() {
        i.borrow_mut().father = ptr.borrow().father.as_ref().cloned();
    }
    let elder = ptr.borrow_mut().father.take().and_then(|x| x.upgrade());
    if elder.is_some() {
        let elder = elder.unwrap();
        let mut father = elder.borrow_mut();
        let left = father.children[0].as_ref().map(|x| x.as_ptr()).unwrap_or(0 as _);
        if left == addr {
            father.children[0] = replacement.clone();
        } else {
            father.children[1] = replacement.clone();
        }
    }

    if let Some(true) = root.as_ref().map(|x| x.as_ptr() == ptr.as_ptr()) {
        *root = replacement;
    }
    ptr
}


impl<T: Debug> Debug for BinaryNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self.children {
            [Some(a), Some(b)] => {
                write!(f, "[{:?}, {:?}, {:?}]", *a.borrow(), self.element, *b.borrow())
            }
            [Some(a), None] => {
                write!(f, "[{:?}, {:?}, _]", *a.borrow(), self.element)
            }
            [None, Some(b)] => {
                write!(f, "[_, {:?}, {:?}]", self.element, *b.borrow())
            }
            [None, None] => {
                write!(f, "[_, {:?}, _]", self.element)
            }
        }
    }
}

pub struct BinaryTree<T> {
    root: Option<Ptr<T>>
}

impl<T: PartialOrd> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree {
            root: None
        }
    }

    fn insert(&mut self, x: T) {
        insert(&mut self.root, x, None);
    }

    fn remove(&mut self, x: T) -> bool {
        let node = locate(&mut self.root, x);
        node.map(|x| remove(x, &mut self.root)).is_some()
    }
}

impl<T : Debug> Debug for BinaryTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self.root {
            Some(root) => write!(f, "{:?}", *root.borrow()),
            None => write!(f, "EMPTY")
        }
    }
}

#[cfg(test)]
mod bintree_bench {
    use test::Bencher;
    use rand::Rng;
    use rand::prelude::{SliceRandom, StdRng};

    #[bench]
    fn random_insertion_10000(bencher: &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(crate::SEED);
        bencher.iter(|| {
            let mut tree= super::BinaryTree::new();
            for _ in 0..10000 {
                tree.insert(rng.gen::<usize>());
            }
        });
    }

    #[bench]
    fn random_insert_then_remove_10000(bencher: &mut Bencher) {
        let mut rng : StdRng = rand::SeedableRng::from_seed(crate::SEED);
        let mut data = Vec::new();
        for _ in 0..10000 {
            data.push(rng.gen::<usize>());
        }
        let mut codata = data.clone();
        codata.shuffle(&mut rng);
        bencher.iter(|| {
            let mut tree= super::BinaryTree::new();
            for i in &data {
                tree.insert(i);
            }
            for i in &codata {
                tree.remove(i);
            }
        });
    }

}

