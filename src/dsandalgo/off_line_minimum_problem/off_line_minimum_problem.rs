use super::{utils::{Sequence, SequenceItem}, reduction::reduce, disjoint_set::DisjointSet};

pub fn offline_minimum_problem(sequence: &Sequence)->Vec<u32>{
    let mut sequence_copy = vec![SequenceItem::E; sequence.len()];
    sequence_copy.copy_from_slice(sequence);

    let reduction = reduce(sequence_copy);

    let maximun = match reduction.iter().filter(
        |item| {
            match item {
                &&SequenceItem::E=>false,
                &&SequenceItem::I(_)=>true
            }
        }
    ).max_by_key(
        |value| {
            match value {
                &&SequenceItem::E=>0, // this will not exist on the list
                &&SequenceItem::I(number)=>number
            }
        }
    ).unwrap() {
        &SequenceItem::E=>0,
        &SequenceItem::I(value)=>value
    };
    let set_count: u32 = reduction.iter().fold(
        0, |accumulator, value|{
            match value{
                &SequenceItem::E=> accumulator+1,
                &SequenceItem::I(_)=>accumulator
            }
        }
    );
    let mut disjoint_set = DisjointSet::build_disjoint_set(
        &reduction, maximun as usize
    );
    let mut result = vec![0; set_count as usize];
    for number in 1..maximun{
        let number_size = number as usize;
        let set_number = disjoint_set.find_set_number(number_size);

        if set_number != set_count + 1{
            result[set_number as usize] = number;
            disjoint_set.merge_sets(number_size);
        }
    }
    result
}