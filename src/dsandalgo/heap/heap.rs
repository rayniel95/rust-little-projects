use std::borrow::BorrowMut;
use std::usize;
use std::{rc::{Rc, Weak}, cell::RefCell};
use std::mem;

type HeapPointer<T> = Rc<RefCell<Heap<T>>>;
type LinkedNodePointer<T> = Rc<RefCell<LinkedNode<T>>>;
type HeapWeakPointer<T> = Weak<RefCell<Heap<T>>>;
type LinkedNodeWeakPointer<T> = Weak<RefCell<LinkedNode<T>>>;




struct Cell<T>{
    value:T,
    priority: u32
}

struct LinkedNode<T>{
    next: Option<LinkedNodePointer<T>>,
    prev: Option<LinkedNodeWeakPointer<T>>,
    value: HeapPointer<T>
}

pub struct HeapTree<T>{
    start: Option<LinkedNodeWeakPointer<T>>,
    end: Option<LinkedNodePointer<T>>,
    parentOfLast: Option<LinkedNodeWeakPointer<T>>,
    len: u32
}

struct Heap<T>{
    cell: Cell<T>,
    left: Option<HeapPointer<T>>,
    right: Option<HeapPointer<T>>,
    parent: Option<HeapWeakPointer<T>>,
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
                    mem::swap(&mut (*pointer).borrow_mut().cell, &mut self.cell);
                }
                (*pointer).borrow_mut().heapify_up();
            }
        }
    }
    fn heapify_down(& mut self){
        match &self.left {
            None=>{}
            Some(left_pointer_ref)=>{
                let left_pointer = Rc::clone(left_pointer_ref);
                match &self.right{
                    None=>{
                        if self.cell.priority> left_pointer.borrow().cell.priority{
                            mem::swap(
                                & mut self.cell,
                                &mut (*left_pointer).borrow_mut().cell
                            );
                            (*left_pointer).borrow_mut().heapify_down();
                        }
                    }
                    Some(right_pointer_ref)=>{
                        let mut min: HeapPointer<T>;
                        if left_pointer.borrow().cell.priority <= right_pointer_ref.borrow().cell.priority{
                            min = Rc::clone(left_pointer_ref);
                        } else{
                            min = Rc::clone(right_pointer_ref);
                        }

                        if self.cell.priority > min.borrow().cell.priority{
                            mem::swap(& mut self.cell, &mut (*min).borrow_mut().cell);
                            (*min).borrow_mut().heapify_down();
                        }
                    }
                }
            }
        }
    }
    fn add_left_son(node: HeapPointer<T>, son: HeapPointer<T>){
        (*node).borrow_mut().left = Some(
            Rc::clone(&son)
        );
        (*son).borrow_mut().parent = Some(
            Rc::downgrade(&node)
        );
    }
    fn add_right_son(node: HeapPointer<T>, son: HeapPointer<T>){
        (*node).borrow_mut().right = Some(
            Rc::clone(&son)
        );
        (*son).borrow_mut().parent = Some(
            Rc::downgrade(&node)
        );
    }
}

impl<T> LinkedNode<T> {
    fn new(heap: HeapPointer<T>)->LinkedNode<T>{
        LinkedNode{
            prev: None,
            next: None,
            value: heap
        }
    }
}

impl<T> HeapTree<T>{
    fn add_next_to_node(node: LinkedNodePointer<T>, to_add: LinkedNode<T>){
        let pointer = Rc::new(RefCell::new(to_add));
        (*node).borrow_mut().next=Some(Rc::clone(&pointer));
        (*pointer).borrow_mut().prev = Some(Rc::downgrade(&node));
    }
    fn new()-> HeapTree<T>{
        HeapTree { start: None, end: None, parentOfLast: None, len: 0 }
    }

    fn add(&mut self, value: T, priority: u32){
        let link_to_heap = Rc::new(
            RefCell::new(Heap::new(value, priority))
        );
        let link_to_linkednode = Rc::new(
            RefCell::new(LinkedNode::new(Rc::clone(&link_to_heap)))
        );
        match &self.end{
            None=>{
                self.end = Some(Rc::clone(&link_to_linkednode));
                self.start = Some(Rc::downgrade(&link_to_linkednode));
                self.len=1;
            }
            Some(pointer)=>{
                match &self.parentOfLast{
                    None=>{
                        let mut end = self.end.take().unwrap();
                        Heap::add_left_son(
                            Rc::clone(&(*end).borrow_mut().value),
                            Rc::clone(&link_to_heap)
                        );

                    }
                    Some(parent)=>{

                    }
                }
            }
        }
    }
}