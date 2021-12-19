use std::usize;
use std::{rc::{Rc, Weak}, cell::RefCell};
use std::mem;

struct Cell<T>{
    value:T,
    priority: u32
}

struct LinkedNode<T>{
    next: Option<Rc<RefCell<LinkedNode<T>>>>,
    prev: Option<Weak<RefCell<LinkedNode<T>>>>,
    value: Heap<T>
}

pub struct HeapTree<T>{
    start: Option<Weak<RefCell<LinkedNode<T>>>>,
    end: Option<Rc<RefCell<LinkedNode<T>>>>,
    parentOfLast: Option<Weak<RefCell<LinkedNode<T>>>>,
    len: u32
}

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

    fn heapify_up(& mut self){
        match &self.parent{
            None => {}
            Some(reference)=>{
                let pointer = match reference.upgrade(){
                    None => {todo!("return some error")}
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
        match &self.left {
            None=>{}
            Some(left_pointer)=>{
                match &self.right{
                    None=>{
                        if self.cell.priority> left_pointer.borrow().cell.priority{
                            mem::swap(& mut self.cell, &mut left_pointer.borrow_mut().cell);
                            left_pointer.borrow_mut().heapify_down();
                        }
                    }
                    Some(right_pointer)=>{
                        let mut min: Rc<RefCell<Heap<T>>>;
                        if left_pointer.borrow().cell.priority <= right_pointer.borrow().cell.priority{
                            min = Rc::clone(&left_pointer);
                        } else{
                            min = Rc::clone(&right_pointer);
                        }

                        if self.cell.priority> min.borrow().cell.priority{
                            mem::swap(& mut self.cell, &mut min.borrow_mut().cell);
                            min.borrow_mut().heapify_down();
                        }
                    }
                }
            }
        }
    }
}

impl<T> LinkedNode<T> {
    fn new(heap: Heap<T>)->LinkedNode<T>{
        LinkedNode{
            prev: None,
            next: None,
            value: heap
        }
    }
}

impl<T> HeapTree<T>{
    fn add_next_to_node(node: Rc<RefCell<LinkedNode<T>>>, to_add: LinkedNode<T>){
        let pointer = Rc::new(RefCell::new(to_add));
        node.borrow_mut().next=Some(Rc::clone(&pointer));
        pointer.borrow_mut().prev = Some(Rc::downgrade(&node));
    }
    
    fn new()-> HeapTree<T>{
        HeapTree { start: None, end: None, parentOfLast: None, len: 0 }
    }

    fn add(value: T, priority: u32){
        
    }
}