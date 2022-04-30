use super::heap::HeapTree;
use rand::Rng;

#[cfg(test)]
mod test {
    use core::num;
    use std::{cmp::Ordering, vec};

    use super::*;

    #[test]
    fn fuzzy_test(){
        let mut my_heap = HeapTree::<u32>::new();
        let mut gen = rand::thread_rng();
        let times = 10000000;
        for _ in 0..times{
            let number = gen.gen::<u32>();
            my_heap.add(number, number);
        }
        print!("**********");
        let mut min = my_heap.pop().unwrap();
        for _ in 1..times{
            let mut new_min = my_heap.pop().unwrap();

            if new_min < min{
                println!("{:?} is grater than {:?}", min, new_min);
                panic!();
            }
            min = new_min;
        }
        
    }
    // #[test]
    // fn add_fuzzy_test(){
    //     let mut my_heap = HeapTree::<u32>::new();
    //     let mut gen = rand::thread_rng();
    //     let mut numbers = Vec::new();

    //     for time in 0..100000{
    //         println!("{:?}", time);
    //         let number = gen.gen::<u32>();
    //         numbers.push(number);
    //         my_heap.add(number, number);
    //     }
    //     // println!("{:?}", numbers);
    // }
    // #[test]
    // fn pop_fuzzy_test(){
    //     let mut my_heap = HeapTree::<u32>::new();
    //     let mut gen = rand::thread_rng();
    //     let mut numbers = Vec::new();

    //     let times = 100000;

    //     for _ in 0..times{
    //         let number = gen.gen::<u32>();
    //         numbers.push(number);
    //         my_heap.add(number, number);
    //     }

    //     for time in 0..times{
    //         println!("{:?}", time);
    //         my_heap.pop();
    //     }
    // }
}