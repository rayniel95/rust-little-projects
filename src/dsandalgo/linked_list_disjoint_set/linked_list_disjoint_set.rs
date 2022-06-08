use std::{rc::Rc, cell::RefCell};

type SetLink = Rc<RefCell<NodeSet>>;

struct NodeSet{
    next: Option<SetLink>,
    head: Option<SetLink>,
    length: u32
}

// TODO - implement drop

impl NodeSet {
    fn new()->Self{
        NodeSet{
            next: None,
            head: None,
            length: 0
        }
    }
}

trait SetLinked {
    fn add(&mut self, other: &Self)->bool;
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
}

struct DisjointSet{
    array: Vec<SetLink>
}

impl DisjointSet {
    fn new(lenght: usize)->Self{
        return (0..lenght).map(
            |index|{
                Rc::new(Ref::new(NodeSet::))
            }
        )
    }
}