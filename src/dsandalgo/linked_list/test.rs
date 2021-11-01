use super::linked_list::LinkedList as LinkedList;

#[cfg(test)]
mod test {
    use std::{cell::Ref, cmp::Ordering, mem::take, ops::AddAssign};

    use super::*;

    fn create_list_with_five_values()->LinkedList<i32>{
        let mut list = LinkedList::<i32>::new();
        
        list.add_last(3);
        list.add_last(2);
        list.add_last(7);
        list.add_last(9);
        list.add_last(1); 

        list
    }

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
        let vector = vec![2, 5, 6, 3, 7];
        
        let mut list = LinkedList::<i32>::new();
        for element in &vector{
            list.add_last(*element);
        }
        assert_eq!((&list).peek_first().unwrap().cmp(&vector[0]), Ordering::Equal);
        
        let mut index = 1;
        while let Ok(_) = (& mut list).pop_first() {
            match (&list).peek_first() {
                None=> break,
                Some(result)=> {
                    assert_eq!(result.cmp(&vector[index]), Ordering::Equal);
                    index+=1;
                }
            }
        }
        // let mut index = 0;
        // while let Some(result) = (&list).peek_first(){
        //     assert_eq!(result.cmp(&vector[index]), Ordering::Equal);
            
        //     match (& mut list).pop_first(){
        //         _=>()
        //     }
        //     index +=1;
        // }

        assert_eq!(index, vector.len());
        assert_eq!(list.count(), 0);
    }
}