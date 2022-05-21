use std::cell::RefCell;
use std::rc::Rc;

type SetLink<T> = Rc<RefCell<Set<T>>>;

trait DisjointSetable<T> {
    fn find_set(&mut self)->Self;
    fn merge(& mut self, other: &mut Self);
}

impl<T> DisjointSetable<T> for SetLink<T>{
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

pub struct Set<T>{
    value: T,
    parent: Option<SetLink<T>>,
    rank: u32,
    index: u32,
}

impl<T> Set<T>{
    fn new(value: T, index: u32)-> Self{
        Self{
            value: value,
            parent: None,
            rank:0,
            index: index,
        }
    }
}

pub struct DisjointSet<T>{
    array: [SetLink<T>]
}

impl <T>  DisjointSet<T>{
    pub fn new(size: u32)->Self{
        Self{
            array: [SetLink<T>; size]
        }
    }
    pub fn find_set(index: u32)->u32{

    }
    pub fn merge(index1: u32, index2: u32){

    }
}