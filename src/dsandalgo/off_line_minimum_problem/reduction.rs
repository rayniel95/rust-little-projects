use std::{mem::min_align_of_val, vec};

use super::utils::{
    Sequence, SecuenceItem
};

// NOTE - I am assuming positive numbers but I think, a generalization to 
// Z continuous numbers is possible.
/*
    transform from a sequence of insertions and extractions with, possibility
    repeated values, to a sequence accepted by the problem
*/
pub fn reduce(secuence: Sequence)->Sequence{
    let maximun = match secuence.into_iter().filter(
        |item| {
            match item {
                &SecuenceItem::E=>false,
                &SecuenceItem::I(_)=>true
            }
        }
    ).max_by_key(
        |value| {
            match value {
                &SecuenceItem::E=>0, // this will not exist on the list
                &SecuenceItem::I(number)=>number
            }
        }
    ).unwrap() {
        SecuenceItem::E=>0,
        SecuenceItem::I(value)=>value
    };

    let minimun = match secuence.into_iter().filter(
        |item| {
            match item {
                &SecuenceItem::E=>false,
                &SecuenceItem::I(_)=>true
            }
        }
    ).min_by_key(
        |value| {
            match value {
                &SecuenceItem::E=>0, // this will not exist on the list
                &SecuenceItem::I(number)=>number
            }
        }
    ).unwrap(){
        SecuenceItem::E=>0,
        SecuenceItem::I(value)=>value
    };

    let repeated_times = vec![0; maximun.into() - minimun.into()];

    secuence.into_iter().for_each(
        |value| {
            match value{
                SecuenceItem::I(number)=> repeated_times[
                        number.into() - minimun
                    ]+=1,
                SecuenceItem::E=>{}
            }
        }
    );
    let accumulative_sum = vec![0; repeated_times.len()];
    accumulative_sum[0] = 0;

    for index in 1..repeated_times.len(){
        accumulative_sum[index] = accumulative_sum[index -1] + repeated_times[index-1];
    }

    let mut counter =  vec![1; repeated_times.len()];
    secuence.into_iter().map(
        |value|{
            match value{
                SecuenceItem::E=>SecuenceItem::E,
                SecuenceItem::I(number)=>{
                    let result = accumulative_sum[
                            number.into()-minimun
                        ]+counter[number.into()-minimun];
                    counter[number.into() - minimun]+=1;
                    SecuenceItem::I(result)
                }
            }
        }
    ).collect::<Sequence>()
}