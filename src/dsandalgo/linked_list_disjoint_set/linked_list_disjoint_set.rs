use std::{cell::RefCell, rc::Rc, mem};

type SetLink = Rc<RefCell<NodeSet>>;

struct NodeSet {
    next: Option<SetLink>,
    head: Option<SetLink>,
    length: u32,
    index: usize,
}

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
        let head = &self.array[index].borrow_mut().head;
        match head {
            None => index,
            Some(first) => {
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
                Some(mut next_pointer) => {
                    pointer.borrow_mut().next = Some(Rc::clone(&next_pointer));
                    mem::swap(&mut next_pointer, &mut pointer);
                }
            }
        }
    }
}

impl Drop for DisjointSet {
    fn drop(&mut self) {
        // for node in self.array.iter_mut(){
        //     let head = node.borrow_mut().head.take();
        //     match head {
        //         None=>{
        //             let mut pointer = Rc::clone(node);
        //             loop {            
        //                 let next = pointer.borrow_mut().next.take();
        //                 print!("{} ", pointer.borrow().index);
        //                 match next {
        //                     None => {
        //                         break;
        //                     }
        //                     Some(mut next_pointer) => {
        //                         pointer.borrow_mut().next = Some(Rc::clone(&next_pointer));
        //                         mem::swap(&mut next_pointer, &mut pointer);
        //                     }
        //                 }
        //             }
        //             println!();
        //         }
        //         Some(_)=>{}
        //     }
        // }
        for node in self.array.iter_mut(){
            node.borrow_mut().head.take();
            node.borrow_mut().next.take();
        }
    }
}