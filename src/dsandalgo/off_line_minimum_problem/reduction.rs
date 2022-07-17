use super::utils::{
    Sequence, SequenceItem
};

// NOTE - I am assuming positive numbers but I think, a generalization to 
// Z continuous numbers is possible.
/*
    transform from a sequence of insertions and extractions with, possibility
    repeated values, to a sequence accepted by the problem
*/
pub fn reduce(secuence: Sequence)->Sequence{
    let maximun = match secuence.iter().filter(
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

    let minimun = match secuence.iter().filter(
        |item| {
            match item {
                &&SequenceItem::E=>false,
                &&SequenceItem::I(_)=>true
            }
        }
    ).min_by_key(
        |value| {
            match value {
                &&SequenceItem::E=>0, // this will not exist on the list
                &&SequenceItem::I(number)=>number
            }
        }
    ).unwrap(){
        &SequenceItem::E=>0,
        &SequenceItem::I(value)=>value
    };

    let mut repeated_times = vec![0; maximun as usize - minimun as usize];

    secuence.iter().for_each(
        |value| {
            match value{
                &SequenceItem::I(number)=> repeated_times[
                        number as usize - minimun as usize
                    ]+=1,
                &SequenceItem::E=>{}
            }
        }
    );
    let mut accumulative_sum = vec![0; repeated_times.len()];
    accumulative_sum[0] = 0;

    for index in 1..repeated_times.len(){
        accumulative_sum[index] = accumulative_sum[index -1] + repeated_times[index-1];
    }

    let mut counter =  vec![1; repeated_times.len()];
    secuence.iter().map(
        |value|{
            match value{
                &SequenceItem::E=>SequenceItem::E,
                &SequenceItem::I(number)=>{
                    let result = accumulative_sum[
                            number as usize-minimun as usize
                        ]+counter[number as usize-minimun as usize];
                    counter[number as usize - minimun as usize]+=1;
                    SequenceItem::I(result)
                }
            }
        }
    ).collect::<Sequence>()
}