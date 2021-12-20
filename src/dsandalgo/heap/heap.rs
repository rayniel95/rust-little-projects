use std::borrow::BorrowMut;
use std::usize;
use std::{rc::{Rc, Weak}, cell::RefCell};
use std::mem;

type HeapPointer<T> = Rc<RefCell<Heap<T>>>;
type LinkedNodePointer<T> = Rc<RefCell<LinkedNode<T>>>;
type HeapWeakPointer<T> = Weak<RefCell<Heap<T>>>;
type LinkedNodeWeakPointer<T> = Weak<RefCell<LinkedNode<T>>>;

trait Heapable<T> where Self: Sized {
    fn has_left_child(&self)->bool;
    fn has_right_child(&self)->bool;
    fn add_left_son(&mut self, son: &Self);
    fn add_right_son(&mut self, son: &Self);
    fn pop_right_son(&mut self) ->Option<Self>;
    fn pop_left_son(&mut self)->Option<Self>;
    fn is_right_child(&self, other: &Self)->bool;
    fn is_left_child(&self, other: &Self)->bool;
}

impl<T> Heapable<T> for HeapPointer<T>{
    fn is_right_child(&self, other: &Self)->bool{
        match &self.borrow().right {
            None=> false,
            Some(son)=>{
                Rc::ptr_eq(son, other)
            }
        }
    }
    fn is_left_child(&self, other: &Self)->bool{
        match &self.borrow().left {
            None=> false,
            Some(son)=>{
                Rc::ptr_eq(son, other)
            }
        }
    }

    fn has_left_child(&self) ->bool {
        if let Some(_) = self.borrow().left{
            return true;
        }
        false
    }
    fn has_right_child(&self) ->bool {
        if let Some(_) = self.borrow().right{
            return true;
        }
        false
    }

    fn add_left_son(&mut self, son: &HeapPointer<T>){
        let pointer_son = Rc::clone(son);
        let node = Rc::clone(self);
        (*node).borrow_mut().left = Some(
            Rc::clone(son)
        );
        (*pointer_son).borrow_mut().parent = Some(
            Rc::downgrade(&node)
        );
    }
    fn add_right_son(&mut self, son: &HeapPointer<T>){
        let pointer_son = Rc::clone(son);
        let node = Rc::clone(self);
        (*node).borrow_mut().right = Some(
            Rc::clone(son)
        );
        (*pointer_son).borrow_mut().parent = Some(
            Rc::downgrade(&node)
        );
    }
    fn pop_left_son(&mut self) ->Option<Self> {
        let mut pointer = Rc::clone(self);
        let result = match (*pointer).borrow_mut().left.take(){
            None => None,
            Some(son)=>{
                (*son).borrow_mut().parent=None;
                Some(son)
            }
        }; result
    }
    fn pop_right_son(&mut self) ->Option<Self> {
        let mut pointer = Rc::clone(self);
        let result = match (*pointer).borrow_mut().right.take(){
            None => None,
            Some(son)=>{
                (*son).borrow_mut().parent=None;
                Some(son)
            }
        }; result
    }
}

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

trait LinkedNodable<T> where Self: Sized{
    fn add_next(&mut self, to_add: &Self);
    fn get_next(&mut self)->Option<Self>;
    fn pop_next(&mut self)->Option<Self>;
}

impl<T> LinkedNodable<T> for LinkedNodePointer<T>{

    fn add_next(&mut self, to_add: &LinkedNodePointer<T>){
        let node = Rc::clone(self);
        let node_to_add = Rc::clone(to_add);
        (*node_to_add).borrow_mut().prev = Some(Rc::downgrade(&node));
        (*node).borrow_mut().next=Some(node_to_add);
    }
    fn get_next(&mut self) ->Option<Self> {
        if let Some(pointer) =  &self.borrow().next{
            return Some(Rc::clone(pointer));
        }
        None
    }
    fn pop_next(&mut self) ->Option<Self> {
        let node = Rc::clone(self);
        let result = match (*node).borrow_mut().next.take() {
            None=>None,
            Some(pointer)=>{
                (*pointer).borrow_mut().prev = None;
                Some(pointer)
            }
        }; result
    }
}

impl<T> HeapTree<T>{
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
        match self.end.take(){
            None=>{
                self.end = Some(Rc::clone(&link_to_linkednode));
                self.start = Some(Rc::downgrade(&link_to_linkednode));
                self.len=1;
            }
            Some(mut pointer)=>{
                match &self.parentOfLast{
                    None=>{
                        (*pointer).borrow_mut().value.add_left_son(&link_to_heap);
                        self.parentOfLast = Some(Rc::downgrade(&pointer));
                    }
                    Some(parent)=>{
                        let mut pointer_to_parent = parent.upgrade().unwrap();
                        if pointer_to_parent.borrow().value.has_right_child(){
                            pointer_to_parent = pointer_to_parent.get_next().unwrap();
                            (*pointer_to_parent).borrow_mut().value.add_left_son(
                                &link_to_heap
                            );
                            self.parentOfLast = Some(Rc::downgrade(&pointer_to_parent));
                        }
                        (*pointer_to_parent).borrow_mut().value.add_right_son(&link_to_heap);
                    }
                };
                pointer.add_next(&link_to_linkednode);
                self.end = Some(link_to_linkednode);
                self.len+=1;
                (*link_to_heap).borrow_mut().heapify_up();
            }
        }
    }
    fn pop(&mut self)->Option<T>{
        match self.end.take(){
            None => None,
            Some(end_pointer)=>{
                match &self.parentOfLast {
                    None=>{
                        self.start.take();
                        self.len-=1;
                        Some(
                            Rc::try_unwrap(Rc::try_unwrap(end_pointer)
                            .ok().unwrap().into_inner().value
                        ).ok().unwrap().into_inner().cell.value)
                    }
                    Some(parent)=>{
                        let mut new_end_pointer = Weak::clone(
                            end_pointer.borrow().prev.as_ref().unwrap()
                        ).upgrade().unwrap();
                        new_end_pointer.pop_next().unwrap();
                        self.end = Some(new_end_pointer);
                        
                        let end_heap_pointer = Rc::clone(
                            &end_pointer.borrow().value
                        );
                        let pointer_to_parent = parent.upgrade().unwrap();
                        if pointer_to_parent.borrow().value.is_left_child(&end_heap_pointer){

                        } else{

                        }

                        None
                    }
                }
            }
        }
    }
}