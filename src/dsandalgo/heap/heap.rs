use std::usize;
use std::{rc::{Rc, Weak}, cell::RefCell};
use std::mem;

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
// TODO - it is necessary the RefCell
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

    fn heapify_up(& mut self){
        match &self.parent{
            None => {}
            Some(reference)=>{
                let pointer = match reference.upgrade(){
                    None => {todo!("throw some error")}
                    Some(pointer)=> pointer
                };

                if pointer.borrow().cell.priority > self.cell.priority{
                    mem::swap(&mut pointer.borrow_mut().cell, &mut self.cell);
                }
                pointer.borrow_mut().heapify_up();
            }
        }
    }
    fn heapify_down(& mut self){
        todo!()
    }
}