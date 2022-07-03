use std::cell::RefCell;
use std::rc::{Rc, Weak};


type StrongLink = Rc<RefCell<NodeSet>>;
type WeakLink = Weak<RefCell<NodeSet>>;

trait DisjointSetable {
    fn find_set(&mut self)->Self;
    fn merge(& mut self, other: &mut Self);
}

impl DisjointSetable for StrongLink{
    fn find_set(&mut self) ->Self {
        if let None = self.borrow().parent {
            return Rc::clone(self);
        }
        self.borrow_mut().parent = Some(
            self.borrow_mut().parent.as_mut().unwrap().find_set()
        );
        Rc::clone(self.borrow().parent.as_ref().unwrap())
    }
    fn merge(& mut self, other: &mut Self) {
        let left = self.find_set();
        let right= other.find_set();

        if left.borrow().rank > right.borrow().rank{
            (*right).borrow_mut().parent = Some(left);
            return;
        }

        if left.borrow().rank == right.borrow().rank{
            (*right).borrow_mut().rank += 1;
        }
        (*left).borrow_mut().parent = Some(right);

    }
}

pub struct NodeSet{
    parent: Option<StrongLink>,
    rank: u32,
    index: usize,
    set_number: u32,
    previous: Option<WeakLink>,
    next: Option<StrongLink>,
}

impl NodeSet{
    fn new(index: usize)-> Self{
        Self{
            parent: None,
            rank:1,
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

pub struct DisjointSet{
    array: Vec<StrongLink>
}

impl  DisjointSet{
    pub fn new(size: usize)->Self{
        Self{ 
            array: (0..size).map(
                |element| Rc::new(RefCell::new(NodeSet::new(element)))
            ).collect()
        }
    }
    pub fn find_set(& mut self, index: usize)->usize{
        self.array[index].find_set().borrow().index
    }
    pub fn merge(&self, index1: usize, index2: usize){
        let mut one = Rc::clone(&self.array[index1]);
        let mut two = Rc::clone(&self.array[index2]);
        one.merge(&mut two)
    }
}

impl Drop for DisjointSet {
    fn drop(&mut self) {
        for set in self.array.iter_mut(){
            set.borrow_mut().parent = None;
        }
    }
}
