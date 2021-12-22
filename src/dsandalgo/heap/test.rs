use super::heap::HeapTree;
use rand::Rng;

#[cfg(test)]
mod test {
    use std::{cmp::Ordering, vec};

    use super::*;

    #[test]
    fn fuzzy_test(){
        let mut my_heap = HeapTree::<u32>::new();
        let mut gen = rand::thread_rng();

        for number in 0..100000{
            let number = gen.gen::<u32>();
            my_heap.add(number, number);
        }
        print!("**********");
        let mut min = my_heap.pop().unwrap();
        for _ in 1..5000{
            let mut new_min = my_heap.pop().unwrap();

            if new_min < min{
                println!("{:?} is grater than {:?}", min, new_min);
                panic!();
            }
            min = new_min;
        }
        
    }
}