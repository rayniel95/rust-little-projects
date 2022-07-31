use super::off_line_minimum_problem::offline_minimum_problem;
use super::utils::SequenceItem;

#[cfg(test)]
mod test {
    use rand::Rng;

    use super::*;

    #[test]
    fn simple_test() {
        let sequence = vec![
            SequenceItem::I(4),
            SequenceItem::I(8),
            SequenceItem::E, // 4
            SequenceItem::I(3), 
            SequenceItem::E, // 3
            SequenceItem::I(9), 
            SequenceItem::I(2), 
            SequenceItem::I(6),
            SequenceItem::E, // 2
            SequenceItem::E, // 6
            SequenceItem::E, // 9
            SequenceItem::I(1), 
            SequenceItem::I(7),
            SequenceItem::E, // 1
            SequenceItem::I(5)
        ];
        let result =  offline_minimum_problem(&sequence);
        assert_eq!(
            vec![4, 3, 2, 6, 9, 1], result
        );
    }
}
