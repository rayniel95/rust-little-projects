use super::linked_list_disjoint_set::DisjointSet;

#[cfg(test)]
mod test {
    use std::{cmp::Ordering, convert::TryInto, vec};

    use rand::Rng;

    use super::*;

    #[test]
    fn fuzzy_test() {
        let nodes = 1000;
        let mut matrix = vec![vec![false; nodes]; nodes];
        let disjoint_set = DisjointSet::new(nodes);
        let times = 1000;
        let mut generator = rand::thread_rng();

        for _ in 0..times {
            let one = generator.gen_range(0..nodes);
            let two = generator.gen_range(0..nodes);

            disjoint_set.merge(
                one,
                two
            );

            matrix[one][two] = true;
            matrix[two][one] = true;

            for index in 0..nodes {
                let expr = 
                    matrix[two][index] || matrix[one as usize][index];

                matrix[one][index] = expr;
                matrix[two][index] = expr;
            }         
        }
        for _ in 0..times {
            let one = generator.gen_range(0..nodes);
            let two = generator.gen_range(0..nodes);

            let one_repr = disjoint_set.find_set(one);
            let two_repr = disjoint_set.find_set(two);

            assert_eq!(matrix[one][two], one_repr==two_repr);       
        }
    }
}
