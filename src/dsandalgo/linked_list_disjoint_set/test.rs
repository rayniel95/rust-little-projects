use super::linked_list_disjoint_set::DisjointSet;

#[cfg(test)]
mod test {
    use std::{cmp::Ordering, convert::TryInto, vec};

    use rand::Rng;

    use super::*;

    #[test]
    fn fuzzy_test() {
        let nodes = 10000;
        let times_to_merge = 5000;

        let mut matrix = vec![vec![false; nodes]; nodes];

        for index in 0..nodes {
            matrix[index][index] = true;
        }
        let disjoint_set = DisjointSet::new(nodes);
        let mut generator = rand::thread_rng();
        println!("before merge");
        for _ in 0..times_to_merge {
            let one = generator.gen_range(0..nodes);
            let two = generator.gen_range(0..nodes);

            disjoint_set.merge(one, two);

            matrix[one][two] = true;
            matrix[two][one] = true;

            for elem in 0..nodes {
                if matrix[elem][one] || matrix[elem][two] {
                    for other in 0..nodes {
                        matrix[elem][other] = matrix[one][other] ||
                            matrix[two][other]
                    }
                }
            }
        }
        println!("to check");
        for _ in 0..times_to_merge {
            let one = generator.gen_range(0..nodes);
            let two = generator.gen_range(0..nodes);

            let one_repr = disjoint_set.find_set(one);
            let two_repr = disjoint_set.find_set(two);

            println!("one:{}-two:{}", one, two);
            println!("one repr:{}-two repr:{}", one_repr, two_repr);

            assert_eq!(matrix[one][two], one_repr == two_repr);
        }
    }
}
