use std::usize;

struct Cell<T>{
    value:T,
    priority: u32
}

pub struct Heap<T>{
    backend: Vec<Cell<T>>
}

// TODO - copy the array if it is not large enough
// NOTE - check the capacity and the lenght for modifications
impl<T> Heap<T> {
    pub fn new(capacity: usize)->Heap<T>{
        Heap{
            backend: Vec::with_capacity(capacity)
        }
    }

    // pub fn peek() -> &T{

    // }
    // pub fn pop_min()->T{

    // }
    // pub fn add(value: T, priority: u32){

    // }
    // pub fn set_priority(index: u32, priority: u32){

    // }

    // pub fn get_index(value: T)->Vec<u32>{

    // }
}