use std::{cell::RefCell, rc::Rc};

type SetLink = Rc<RefCell<NodeSet>>;

struct NodeSet {
    next: Option<SetLink>,
    head: Option<SetLink>,
    length: u32,
    index: usize,
}

// TODO - implement drop

impl NodeSet {
    fn new(index: usize) -> Self {
        NodeSet {
            next: None,
            head: None,
            length: 1,
            index,
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

pub struct DisjointSet {
    array: Vec<SetLink>,
}

impl DisjointSet {
    pub fn new(lenght: usize) -> Self {
        DisjointSet {
            array: (0..lenght)
                .map(|index| Rc::new(RefCell::new(NodeSet::new(index))))
                .collect(),
        }
    }

    pub fn find_set(&self, index: usize) -> usize {
        match self.array[index].borrow_mut().head.take() {
            None => index,
            Some(first) => {
                self.array[index].borrow_mut().head = Some(Rc::clone(&first));
                first.borrow().index
            }
        }
    }
    pub fn merge(&self, index1: usize, index2: usize) {
        let one = self.find_set(index1);
        let two = self.find_set(index2);

        if one == two {
            return;
        }

        let lenght_one = self.array[one].borrow().length;
        let lenght_two = self.array[two].borrow().length;

        if lenght_one > lenght_two {
            self.array[one].borrow_mut().length = lenght_one + lenght_two;
            self.insert(one, two);
            return;
        }
        self.array[two].borrow_mut().length = lenght_one + lenght_two;
        self.insert(two, one);
    }
    fn insert(&self, one: usize, two: usize) {
        let last = self.array[one].borrow_mut().next.take();
        self.array[one].borrow_mut().next = Some(Rc::clone(&self.array[two]));
        self.array[two].borrow_mut().length = 1;

        let mut pointer = Rc::clone(&self.array[two]);
        loop {
            pointer.borrow_mut().head = Some(Rc::clone(&self.array[one]));
            pointer.borrow_mut().length = 1;

            let next = pointer.borrow_mut().next.take();
            match next {
                None => {
                    pointer.borrow_mut().next = last;
                    break;
                }
                Some(next_pointer) => {
                    let temp = pointer;
                    pointer = Rc::clone(&next_pointer);
                    temp.borrow_mut().next = Some(Rc::clone(&pointer));
                }
            }
        }
    }
}

impl Drop for DisjointSet {
    fn drop(&mut self) {
        for node in self.array.iter_mut(){
            node.borrow_mut().head.take();
            node.borrow_mut().next.take();
        }
    }
}