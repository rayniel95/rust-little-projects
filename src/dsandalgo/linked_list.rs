use std::borrow::{Borrow, BorrowMut};
use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref, RefMut, UnsafeCell};

type LinkToNode<T> = Rc<RefCell<LinkedNode<T>>>;

fn newLinkToNode<T>(value: T) -> LinkToNode<T>{
    let new_node = Rc::new(RefCell::new(LinkedNode::new(value)));
    new_node
}

struct LinkedNode<T>{
    pub value: T,
    pub next: Option<LinkToNode<T>>
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
    pub fn addLast(& mut self, value: T){
        if self.count == 0{
            self.count +=1;
            let new_node = newLinkToNode(value);
            self.start = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
            return;
        }
        self.count+=1;
        let new_node = newLinkToNode(value);
        let tail = match self.tail.take(){
            Some(value)=> value,
            None => panic!(
                "lenght greater or equal than one and tail ponting to None"
            ) // TODO - trt to create a error for this
        };
        tail.borrow_mut().next = Some(Rc::clone(&new_node));
        self.tail = Some(Rc::clone(&new_node));
    }

    pub fn popFirst(& mut self) -> Result<T, LinkedListError>{
        match self.start.take(){
            Some(pointer) => {
                match pointer.borrow_mut().next.take(){
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
    pub fn addFirst(& mut self, value: T){
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
    pub fn popLast(& mut self) -> Result<T, LinkedListError>{
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
        // TODO - try to do this without use a rc pointer for iterate
        let mut pointer = self.start.take().unwrap();
        self.start = Some(Rc::clone(&pointer));
        for _ in 1..self.count {
            pointer = match (*pointer).borrow().next {
                Some(value) =>{
                    value
                }
                None=>{
                    break;
                }
            };
        }
        let result = (*pointer).borrow().next.take().unwrap();
        self.tail = Some(pointer);

        Ok(Rc::try_unwrap(result).ok().unwrap().into_inner().value)
    }
}

// TODO - implement iterable