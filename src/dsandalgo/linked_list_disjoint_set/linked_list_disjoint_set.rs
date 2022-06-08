use std::{rc::Rc, cell::RefCell};

type SetLink = Rc<RefCell<NodeSet>>;

struct NodeSet{
    next: Option<SetLink>,
    head: Option<SetLink>,
    length: Option<u32>
}

// TODO - implement drop

trait SetLinked {
    fn add(&mut self, other: &Self)->bool;
    fn introduce(&mut self, other: &Self);
}

impl SetLinked for SetLink {
    fn add(&mut self, other: &Self) ->bool {
        match self.borrow_mut().next.take() {
            Some(next)=> {
                self.borrow_mut().next = Some(Rc::clone(&next));
                false
            },
            None=>{
                self.borrow_mut().next = Some(Rc::clone(other));
                true
            }
        }
    }
    fn introduce(&mut self, other: &Self) {
        
    }
}