use std::cell::RefCell;
use std::rc::{Rc, Weak};

type StrongLink = Rc<RefCell<NodeSet>>;
type WeakLink = Weak<RefCell<NodeSet>>;

pub struct NodeSet {
    parent: Option<StrongLink>,
    rank: u32,
    index: usize,
    set_number: u32,
    previous: Option<WeakLink>,
    next: Option<StrongLink>,
}

impl NodeSet {
    fn new(index: usize) -> Self {
        Self {
            parent: None,
            rank: 1,
            index,
            set_number: 1,
            previous: None,
            next: None,
        }
    }
}

impl Default for NodeSet {
    fn default() -> Self {
        NodeSet::new(0)
    }
}

pub struct DisjointSet {
    array: Vec<StrongLink>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        Self {
            array: (0..size)
                .map(|element| Rc::new(RefCell::new(NodeSet::new(element))))
                .collect(),
        }
    }

    pub fn find_set(&mut self, index: usize) -> usize {
        if let None = self.array[index].borrow().parent {
            return index;
        }
        let parent_index = 
            self.array[index].borrow().parent.unwrap().borrow().index;

        self.array[index].borrow_mut().parent =
            Some(Rc::clone(&self.array[self.find_set(parent_index)]));
        self.array[index].borrow().parent.unwrap().borrow().index
    }

    pub fn merge(&self, index1: usize, index2: usize) {
        let left = self.find_set(index1);
        let right = self.find_set(index2);

        if self.array[left].borrow().rank > self.array[right].borrow().rank {
            self.array[right].borrow_mut().parent = Some(
                Rc::clone(&self.array[left])
            );
            return;
        }

        if self.array[left].borrow().rank == self.array[right].borrow().rank {
            self.array[right].borrow_mut().rank += 1;
        }
        self.array[left].borrow_mut().parent = Some(
            Rc::clone(&self.array[right])
        );
    }
}

impl Drop for DisjointSet {
    // FIXME - finish this
    fn drop(&mut self) {
        for set in self.array.iter_mut() {
            set.borrow_mut().parent = None;
        }
    }
}
