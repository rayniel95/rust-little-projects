use std::ops::Index;
use std::rc::{Rc, Weak};
use std::cell::{Ref, RefCell};

type LinkToNode<T> = Rc<RefCell<LinkedNode<T>>>;

fn newLinkToNode<T>(value: T) -> LinkToNode<T>{
    let new_node = Rc::new(RefCell::new(LinkedNode::new(value)));
    new_node
}

struct LinkedNode<T>{
    value: T,
    next: Option<LinkToNode<T>>
}

impl<T> LinkedNode<T>{
    fn new(value: T) -> Self {
        LinkedNode{
            value: value,
            next: None
        }
    }
}

enum LinkedListError{
    EmptyList
}

pub struct LinkedList<T>{
    start: Option<LinkToNode<T>>,
    tail: Option<LinkToNode<T>>,
    count: u32
}

impl<T> LinkedList<T>{
    pub fn new() -> Self{
        LinkedList{
            start: None,
            tail: None,
            count: 0
        }
    }

    pub fn count(&self) -> u32{
        self.count
    }
    // TODO - try to use pattern matching
    pub fn add_last(& mut self, value: T){
        if self.count == 0{
            self.count +=1;
            let new_node = newLinkToNode(value);
            self.start = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
            return;
        }
        self.count+=1;
        let new_node = newLinkToNode(value);
        (*self.tail.take().unwrap()).borrow_mut().next = Some(
            Rc::clone(&new_node)
        );
        self.tail = Some(Rc::clone(&new_node));
    }

    pub fn pop_first(& mut self) -> Result<T, LinkedListError>{
        match self.start.take(){
            Some(pointer) => {
                match (*pointer).borrow_mut().next.take(){
                    Some(next) => {
                        self.count -= 1;
                        self.start = Some(
                            Rc::clone(&next)
                        );
                    }
                    None => {
                        self.count = 0;
                        self.tail.take();
                    }
                }; // TODO - try to create an error for this
                Ok(Rc::try_unwrap(pointer).ok().unwrap().into_inner().value)
            }
            None => Err(LinkedListError::EmptyList)
        }
    }
    pub fn add_first(& mut self, value: T){
        let new_node = newLinkToNode(value);
        self.count+=1;
        match self.start.take() {
            Some(pointer) => {
                (*new_node).borrow_mut().next = Some(pointer);
                self.start = Some(Rc::clone(&new_node));
            }
            None => {
                self.start = Some(Rc::clone(&new_node));
                self.tail = Some(Rc::clone(&new_node));
            }
        }
    }
    pub fn pop_last(& mut self) -> Result<T, LinkedListError>{
        if self.count == 0{
            return Err(LinkedListError::EmptyList);
        }
        self.count -= 1;
        if self.count == 0{
            self.tail.take();
            return Ok(
                Rc::try_unwrap(
                    self.start.take().unwrap()
                ).ok().unwrap().into_inner().value);
        };
        // TODO - try to do this without use a rc pointer for iterate, maybe
        // use ref, similar al too many lists donde retornan un ref a un nodo
        // interno
        // TODO - this code is the same in indexer, create a method
        let mut pointer = self.start.take().unwrap();
        self.start = Some(Rc::clone(&pointer));
        for _ in 1..self.count {
            let temp = match &((*pointer).borrow().next) {
                Some(value) =>{
                    Rc::clone(value)
                }
                None=>{
                    break;
                }
            };
            pointer = temp;
        }
        let result = (*pointer).borrow_mut().next.take().unwrap();
        self.tail = Some(pointer);
        // TODO - create error for catch this, write a match
        Ok(Rc::try_unwrap(result).ok().unwrap().into_inner().value)
    }

}

// TODO - implement indexable
// TODO - implement iterable
// TODO - implement Extend<&'a T>, Extend<T>, From<&'_ [T]> for Vec<T, Global>,
// From<&'_ mut [T]> for Vec<T, Global>, From<[T; N]> for Vec<T, Global>,
// From<BinaryHeap<T>> for Vec<T, Global>, From<Box<[T], A>> for Vec<T, A>,
// From<Vec<T, A>> for Box<[T], A>, From<VecDeque<T>> for Vec<T, Global>,
// FromIterator<T> for Vec<T, Global>, From<Box<[T], A>> for Vec<T, A>

// TODO - separate in multiple files the different implementations and tests