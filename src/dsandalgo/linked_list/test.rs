use super::linked_list::LinkedList;
use super::linked_list::LinkedListError;

#[cfg(test)]
mod test {
    use std::{cmp::Ordering};

    use super::*;

    #[test]
    fn add_last_and_count_test(){
        let mut list = LinkedList::<i32>::new();

        list.add_last(3);
        assert_eq!((&list).count(), 1);
        list.add_last(4);
        assert_eq!((&list).count(), 2);
        list.add_last(2);
        assert_eq!((&list).count(), 3);
    }

    #[test]
    fn iterator_test(){
        let vector = vec![2, 5, 6, 3, 7];
        
        let mut list = LinkedList::<i32>::new();
        for element in &vector{
            list.add_last(*element);
        }
        
        let mut index=0;
        for element in list{
            assert_eq!(element, vector[index]);
            index+=1;
        }
    }

    #[test]
    fn peek_first_test(){
        let vector = vec![2, 5, 6, 3, 7, 10, 15];
        
        let mut list = LinkedList::<i32>::new();
        for element in &vector{
            list.add_last(*element);
        }
        
        let mut index = 0;
        loop {
            match (&list).peek_first() {
                None => break,
                Some(result) => {
                    assert_eq!(result.cmp(&vector[index]), Ordering::Equal);
                    index+=1;
                }
            }
            list.pop_first();
        }

        assert_eq!(index, vector.len());
        assert_eq!(list.count(), 0);
    }

    #[test]
    fn pop_first_test() {
        let vector = vec![2, 5, 6, 3, 7, 10, 15];
        
        let mut list = LinkedList::<i32>::new();
        
        for element in &vector{
            list.add_last(*element);
        }

        let mut index=0;
        loop {
            match list.pop_first(){
                Err(_)=>break,
                Ok(element)=> assert_eq!(element, vector[index])
            }
            index+=1;
        }
        assert_eq!((&list).count(), 0);
        assert_eq!(list.pop_first().is_err(), true);
    }

    
}