use std::iter::Inspect;

pub fn solution(prices: Vec<f64>, notes: Vec<String>, x: f64) -> bool {
    let mut in_store = vec![0.0; prices.len()];
    for index in 0..prices.len() {
        let op: Vec<&str> = notes[index].split(" ").collect();
        if op.len() == 3 {
            in_store[index] = prices[index];
        } else if op[1] == "lower" {
            in_store[index] =
                prices[index] * (
                    1.0 - op[0][..op[0].len()-1].parse::<f64>().unwrap() / 100.0
                ).recip()
        } else {
            in_store[index] =
                prices[index] * (
                    1.0 + op[0][..op[0].len()-1].parse::<f64>().unwrap() / 100.0
                ).recip()
        }
    }
    // println!("{:?}", in_store);

    let mut result = 0.0;
    for index in 0..in_store.len() {
        result += prices[index]-in_store[index];
    }
    // println!("{}", result);
    result <= x
}
