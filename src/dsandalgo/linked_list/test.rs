use super::linked_list::LinkedList as LinkedList;

#[cfg(test)]
mod test {
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
        
    }
}