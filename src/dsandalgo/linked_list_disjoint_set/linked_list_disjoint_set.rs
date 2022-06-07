use std::{rc::Rc, cell::RefCell};

type SetLink = Rc<RefCell<Set>>;

struct Set{
    next: Option<SetLink>,
    head:Option<SetLink>
}

struct SetObject{
    tail: SetLink,
    length: u32
}

// TODO - implement drop

trait SetLinked {
    fn add(&mut self, other: &Self)->bool;
    fn introduce(&mut self, other: &Self);
}

impl SetLinked for SetLink {
    fn add(&mut self, other: &Self) ->bool {
        // REVIEW - this can launch error on runtime
        match &mut self.borrow_mut().next.take() {
            Some(_)=> false,
            None=>{
                self.borrow_mut().next = Some(Rc::clone(other));
                true
            }
        }
    }
    fn introduce(&mut self, other: &Self) {
        
    }
}