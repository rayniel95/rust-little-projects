use std::usize;
use std::{rc::{Rc, Weak}, cell::RefCell};

struct Cell<T>{
    value:T,
    priority: u32
}

// struct LinkedNode<T>{

// }

// pub struct HeapTree<T>{

// }

struct Heap<T>{
    cell: Cell<T>,
    left: Option<Rc<RefCell<Heap<T>>>>,
    right: Option<Rc<RefCell<Heap<T>>>>,
    parent: Option<Weak<RefCell<Heap<T>>>>,
}

// NOTE - check the capacity and the lenght for modifications
impl<T> Heap<T> {
    fn new(value:T, priority: u32)->Heap<T>{
        Heap{
            cell: Cell{
                value: value,
                priority: priority
            },
            left: None,
            right: None,
            parent: None
        }
    }

    fn heapifyUp(){
        todo!()
    }
    fn heapifyDown(){
        todo!()
    }
}