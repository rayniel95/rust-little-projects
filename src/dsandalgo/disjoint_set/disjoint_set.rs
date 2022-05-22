use std::cell::RefCell;
use std::rc::Rc;

type SetLink = Rc<RefCell<Set>>;

trait DisjointSetable {
    fn find_set(&mut self)->Self;
    fn merge(& mut self, other: &mut Self);
}

impl DisjointSetable for SetLink{
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

pub struct Set{
    parent: Option<SetLink>,
    rank: u32,
    index: u32,
}

impl Set{
    fn new(index: u32)-> Self{
        Self{
            parent: None,
            rank:0,
            index: index,
        }
    }
}

impl Default for Set {
    fn default() -> Self {
        Set::new(0)
    }
}

pub struct DisjointSet{
    array: Vec<SetLink>
}

impl  DisjointSet{
    pub fn new(size: u32)->Self{
        Self{ 
            array: (0..size).map(
                |element| Rc::new(RefCell::new(Set::new(element)))
            ).collect()
        }
    }
    pub fn find_set(&self, index: u32)->u32{
        
    }
    pub fn merge(&self, index1: u32, index2: u32){

    }
}

// TODO - implement Drop