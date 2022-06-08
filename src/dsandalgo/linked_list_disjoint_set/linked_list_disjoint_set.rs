use std::{cell::RefCell, rc::Rc};

type SetLink = Rc<RefCell<NodeSet>>;

struct NodeSet {
    next: Option<SetLink>,
    head: Option<SetLink>,
    length: u32,
    index: usize
}

// TODO - implement drop

impl NodeSet {
    fn new(index: usize) -> Self {
        NodeSet {
            next: None,
            head: None,
            length: 0,
            index
        }
    }
}

trait SetLinked {
    fn add(&mut self, other: &Self) -> bool;
}

impl SetLinked for SetLink {
    fn add(&mut self, other: &Self) -> bool {
        match self.borrow_mut().next.take() {
            Some(next) => {
                self.borrow_mut().next = Some(Rc::clone(&next));
                false
            }
            None => {
                self.borrow_mut().next = Some(Rc::clone(other));
                true
            }
        }
    }
}

struct DisjointSet {
    array: Vec<SetLink>,
}

impl DisjointSet {
    fn new(lenght: usize) -> Self {
        DisjointSet {
            array: (0..lenght)
                .map(|index| Rc::new(RefCell::new(NodeSet::new(index))))
                .collect(),
        }
    }

    fn find_set(&self, index: usize)->usize{
        match self.array[index].borrow_mut().head.take(){
            None=> index,
            Some(first)=>{
                self.array[index].borrow_mut().head = Some(Rc::clone(&first));
                first.borrow().index
            }
        }
    }
    fn merge(&self, index1: usize, index2: usize){
        let one = self.find_set(index1);
        let two = self.find_set(index2);

        if 

        let lenght_one = self.array[one].borrow().length;
        let lenght_two = self.array[two].borrow().length;

        if lenght_one > lenght_two{
            self.array[one].borrow_mut().length = lenght_one + lenght_two;
        }
    }
}
