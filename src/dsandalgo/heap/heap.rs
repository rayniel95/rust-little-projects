use std::{rc::{Rc, Weak}, cell::RefCell, ptr::NonNull};
use std::mem;

use contracts::*;
use rand::distributions::uniform::SampleBorrow;

type HeapPointer<T> = Rc<RefCell<Heap<T>>>;
type LinkedNodePointer<T> = Rc<RefCell<LinkedNode<T>>>;
type HeapWeakPointer<T> = Weak<RefCell<Heap<T>>>;
type LinkedNodeWeakPointer<T> = Weak<RefCell<LinkedNode<T>>>;
type OptionalLinkedNodePointer<T> = Option<LinkedNodePointer<T>>;
type OptionalHeapPointer<T> = Option<HeapPointer<T>>;

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
    start: Option<LinkedNodePointer<T>>,
    end: Option<LinkedNodePointer<T>>,
    parentOfLast: Option<LinkedNodeWeakPointer<T>>,
    len: u32
}

pub(crate) struct Heap<T>{
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

    #[test_ensures(
        old(self.parent.is_some() && self.cell.priority < self.parent.as_ref().unwrap().upgrade().unwrap().borrow().cell.priority) ->
        self.cell.priority > self.parent.as_ref().unwrap().upgrade().unwrap().borrow().cell.priority
    )]
    #[test_invariant(
        self.parent.is_some() -> self.parent.as_ref().unwrap().upgrade().is_some()
    )]
    #[test_ensures(
        self.parent.is_some() -> old(self.parent.is_some() && self.parent.as_ref().unwrap().upgrade().is_some()) 
    )]
    #[test_ensures(
        self.parent.is_some() -> self.cell.priority >= self.parent.as_ref().unwrap().upgrade().unwrap().borrow().cell.priority
    )]
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
                    (*pointer).borrow_mut().heapify_up();
                }
            }
        }
    }
    fn heapify_down(& mut self){
        match &self.left {
            None=>{
                // println!("cosa");
            }
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
    #[test_requires(
        Rc::clone(self).borrow().next.is_none() && 
        Rc::clone(to_add).borrow().prev.is_none() && 
        Rc::clone(to_add).borrow().next.is_none()
    )]
    #[test_ensures(
        Rc::clone(self).borrow().next.is_some() && 
        Rc::clone(to_add).borrow().prev.is_some() && 
        Rc::clone(to_add).borrow().next.is_none() &&
        Rc::ptr_eq(
            Rc::clone(self).borrow().next.as_ref().unwrap(),
            to_add
        ) &&
        Rc::ptr_eq(
            &Rc::clone(to_add).borrow().prev.as_ref().unwrap().upgrade().unwrap(),
            self
        )
    )]
    fn add_next(&mut self, to_add: &LinkedNodePointer<T>){
        let node = Rc::clone(self);
        let node_to_add = Rc::clone(to_add);
        (*node_to_add).borrow_mut().prev = Some(Rc::downgrade(&node));
        (*node).borrow_mut().next=Some(node_to_add);
    }
    #[test_ensures(
        ret.is_some() -> Rc::ptr_eq(
            self.borrow().next.as_ref().unwrap(),
            ret.as_ref().unwrap()
        )
    )]
    #[test_ensures(
        self.borrow().next.is_none() -> ret.is_none()
    )]
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
    pub fn new()-> HeapTree<T>{
        HeapTree { start: None, end: None, parentOfLast: None, len: 0 }
    }

    // #[test_ensures(
    //     self.start.is_some() -> test_fn::test_heap_structure_and_reachability(&self.start.as_ref().unwrap().borrow().value) as u32 == self.len
    // )]
    // #[test_ensures(
    //     self.start.is_some() -> test_fn::test_heap_partial_ordering(&self.start.as_ref().unwrap().borrow().value)
    // )]
    #[test_invariant(
        // TODO - here to test the linked list structure and that all nodes are 
        // reachables
    )]
    #[test_ensures(
        // TODO - ensure parent of last is in the correct pointer
    )]
    #[test_ensures(
        self.len == old(self.len) + 1
    )]
    #[test_ensures(
        self.start.is_some() && self.end.is_some()
    )]
    #[test_ensures(
        self.len == 1 -> old(self.parentOfLast.is_none()) && self.parentOfLast.is_none()
    )]
    #[test_ensures(
        self.len == 2 -> old(self.parentOfLast.is_none()) && self.parentOfLast.is_some()
    )]
    #[test_ensures(
        old(self.end.is_some()) -> self.end.is_some()
    )]
    // #[test_ensures(
    //     old(self.end.is_some()) -> {
    //         if old(self.end.is_some()){
    //             !Rc::ptr_eq(
    //                 old(
    //                     &match &self.end{
    //                         None=> panic!(
    //                             "aqui se debe devolver alguna especie de direccion de memoria por defecto"
    //                         ),
    //                         Some(pointer) =>{
    //                             Rc::clone(pointer)
    //                         }
    //                     }
    //                 ),
    //                 self.end.as_ref().unwrap()
    //             )
    //         } else{
    //             true
    //         }
    //     }
    // )]
    pub fn add(&mut self, value: T, priority: u32){
        let link_to_heap = Rc::new(
            RefCell::new(Heap::new(value, priority))
        );
        let link_to_linkednode = Rc::new(
            RefCell::new(LinkedNode::new(Rc::clone(&link_to_heap)))
        );
        match self.end.take(){
            None=>{
                self.end = Some(Rc::clone(&link_to_linkednode));
                self.start = Some(Rc::clone(&link_to_linkednode));
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
                        }else{
                            (*pointer_to_parent).borrow_mut().value.add_right_son(&link_to_heap);
                        }
                    }
                };
                pointer.add_next(&link_to_linkednode);
                self.end = Some(link_to_linkednode);
                self.len+=1;
                (*link_to_heap).borrow_mut().heapify_up();
            }
        }
    }
    #[test_ensures(
        self.len == old(self.len) - 1
    )]
    pub fn pop(&mut self)->Option<T>{
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
                        let mut new_end_pointer = 
                            (*end_pointer).borrow_mut().prev.take().unwrap()
                            .upgrade().unwrap();
                        new_end_pointer.pop_next().unwrap();
                        self.end = Some(new_end_pointer);
                        
                        let end_heap_pointer = Rc::try_unwrap(
                    end_pointer).ok().unwrap().into_inner().value;

                        let pointer_to_parent = parent.upgrade().unwrap();
                        if pointer_to_parent.borrow().value.is_left_child(&end_heap_pointer){
                            (*pointer_to_parent).borrow_mut().value.pop_left_son();
                            self.parentOfLast = match &pointer_to_parent.borrow().prev {
                                None=> None,
                                Some(pointer)=>{
                                    Some(
                                        Weak::clone(pointer)
                                    )
                                }  
                            };
                        } else{
                            (*pointer_to_parent).borrow_mut().value.pop_right_son();
                        }
                        self.len-=1;

                        let start = self.start.take().unwrap();
                        mem::swap(
                            &mut (*(*start).borrow_mut().value).borrow_mut().cell,
                            &mut (*end_heap_pointer).borrow_mut().cell
                        );
                        (*(*start).borrow_mut().value).borrow_mut().heapify_down();
                        self.start = Some(start);

                        Some(
                            Rc::try_unwrap(
                                end_heap_pointer).ok().unwrap().into_inner()
                                .cell.value
                        )
                    }
                }
            }
        }
    }
}

impl<T> Drop for HeapTree<T>{
    fn drop(&mut self) {
        match self.end.take() {
            None=>{}
            Some(_)=>{
                self.parentOfLast.take();
                let mut current = self.start.take();
                
                while let Some(pointer_ref) = &current {
                    let mut pointer = Rc::clone(pointer_ref);
                    (*pointer).borrow_mut().value.pop_left_son();
                    (*pointer).borrow_mut().value.pop_right_son();
                    current = pointer.pop_next();
                }
            }
        }
    }
}

mod test_fn {
    use super::*;

    pub(super) fn test_heap_structure_and_reachability<T>(heap: &HeapPointer<T>) -> i32{
        let mut queue = Vec::new();
        let mut counter = 0;
        queue.push(Some(Rc::clone(heap)));

        while let Some(Some(pointer_ref)) = queue.first() {
            let pointer = Rc::clone(pointer_ref);
            queue.remove(0);
            counter +=1;

            match &(*pointer).borrow().left {
                None=> queue.push(None),
                Some(son)=>{
                    queue.push(Some(Rc::clone(son)));
                }
            };
            match &(*pointer).borrow().right {
                None=> queue.push(None),
                Some(son)=>{
                    queue.push(Some(Rc::clone(son)));
                }
            };
        }

        if queue.into_iter().any(|f| f.is_some()){
            return -1;
        }
        counter
    }

    pub(super) fn test_heap_partial_ordering<T>(heap: &HeapPointer<T>) -> bool{
        let pointer = Rc::clone(heap);
        match &(*pointer).borrow().left {
            None=>{}
            Some(son)=>{
                if son.borrow().cell.priority < pointer.borrow().cell.priority ||
                 !test_heap_partial_ordering(son){
                    return  false;
                }
            }
        }
        match &(*pointer).borrow().right {
            None=>{}
            Some(son)=>{
                if son.borrow().cell.priority < pointer.borrow().cell.priority ||
                 !test_heap_partial_ordering(son){
                    return  false;
                }
            }
        }
        true
    }
}