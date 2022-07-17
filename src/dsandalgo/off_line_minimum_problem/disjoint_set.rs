use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use super::utils::{
    Sequence, SequenceItem
};

type StrongLink = Rc<RefCell<NodeSet>>;
type WeakLink = Weak<RefCell<NodeSet>>;

pub struct NodeSet {
    pub parent: Option<StrongLink>,
    pub rank: u32,
    pub index: usize,
    pub set_number: u32,
    pub previous: Option<WeakLink>,
    pub next: Option<StrongLink>,
}

impl NodeSet {
    pub fn new(index: usize) -> Self {
        Self {
            parent: None,
            rank: 1,
            index,
            set_number: 1,
            previous: None,
            next: None,
        }
    }
}

impl Default for NodeSet {
    fn default() -> Self {
        NodeSet::new(0)
    }
}

pub struct DisjointSet {
    array: Vec<StrongLink>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        Self {
            array: (0..size)
                .map(|element| Rc::new(RefCell::new(NodeSet::new(element))))
                .collect(),
        }
    }

    // pub fn find_set(&mut self, index: usize) -> usize {
    //     if let None = self.array[index].borrow().parent {
    //         return index;
    //     }
    //     let parent_index =
    //         self.array[index].borrow().parent.unwrap().borrow().index;

    //     self.array[index].borrow_mut().parent =
    //         Some(Rc::clone(&self.array[self.find_set(parent_index)]));
    //     self.array[index].borrow().parent.unwrap().borrow().index
    // }
    fn find_set(array: &mut Vec<StrongLink>, index: usize) -> usize {
        if let None = array[index].borrow().parent {
            return index;
        }
        let parent_index = array[index].borrow().parent.unwrap().borrow().index;

        (*array[index]).borrow_mut().parent = Some(Rc::clone(
            &array[DisjointSet::find_set(array, parent_index)],
        ));
        array[index].borrow().parent.unwrap().borrow().index
    }

    fn merge(array: &mut Vec<StrongLink>, one: usize, two: usize) {
        let one_repr = DisjointSet::find_set(array, one);
        let two_repr = DisjointSet::find_set(array, two);

        if one_repr == two_repr {
            return;
        }

        if array[one_repr].borrow().rank > array[two_repr].borrow().rank{
            (*array[two_repr]).borrow_mut().parent = Some(
                Rc::clone(&array[one_repr])
            );
            (*array[one_repr]).borrow_mut().set_number = 
                array[two_repr].borrow().set_number;

            (*array[two_repr]).borrow_mut().previous.take();
            let next = 
            (*array[two_repr]).borrow_mut().next.take();

            (*array[one_repr]).borrow_mut().next.take();
            match next {
                None=>{},
                Some(pointer)=>{
                    (*pointer).borrow_mut().previous = Some(
                        Rc::downgrade(&array[one_repr])
                    );
                    (*array[one_repr]).borrow_mut().next = Some(pointer);
                }
            }
            return;
        }
        if array[one_repr].borrow().rank == array[two_repr].borrow().rank{
            (*array[two_repr]).borrow_mut().rank += 1;
        }

        (*array[one_repr]).borrow_mut().parent = Some(
            Rc::clone(&array[two_repr])
        );
        (*array[one_repr]).borrow_mut().next.take();
        (*array[two_repr]).borrow_mut().previous.take();

        let previous =
            (*array[one_repr]).borrow_mut().previous.take();
        match previous{
            None=>{},
            Some(pointer)=>{
                let pointer_up = pointer.upgrade().unwrap();
                (*pointer_up).borrow_mut().next = Some(
                    Rc::clone(&array[two_repr])
                );
                (*array[two_repr]).borrow_mut().previous = Some(pointer);
            }
        }
    }

    pub fn build_disjoint_set(sequence: &Sequence, n: usize)-> Self{
        let mut array = (0..n).map(
            |value| Rc::new(RefCell::new(NodeSet::new(value)))
        ).collect::<Vec<Rc<RefCell<NodeSet>>>>();
        let mut set_number = 1;
        for elem in sequence.iter(){
            match elem {
                SequenceItem::E=>{set_number += 1;}
                &SequenceItem::I(val)=>{
                    (*array[val as usize]).borrow_mut().parent;
                }
            }
        }

        for (elem1, elem2) in sequence.iter().take(
            sequence.len() - 1
        ).zip(sequence.iter().skip(1)){
            match (elem1, elem2){
                (SequenceItem::I(val1), SequenceItem::I(val2))=>{
                    DisjointSet::merge(array, val1, val2);
                }
                _=>{}
            }
        }


    }
    // pub fn merge(&self, index1: usize, index2: usize) {
    //     let left = self.find_set(index1);
    //     let right = self.find_set(index2);

    //     if self.array[left].borrow().rank > self.array[right].borrow().rank {
    //         self.array[right].borrow_mut().parent = Some(
    //             Rc::clone(&self.array[left])
    //         );
    //         return;
    //     }

    //     if self.array[left].borrow().rank == self.array[right].borrow().rank {
    //         self.array[right].borrow_mut().rank += 1;
    //     }
    //     self.array[left].borrow_mut().parent = Some(
    //         Rc::clone(&self.array[right])
    //     );
    // }
}

impl Drop for DisjointSet {
    // FIXME - finish this
    fn drop(&mut self) {
        for set in self.array.iter_mut() {
            set.borrow_mut().parent = None;
        }
    }
}
