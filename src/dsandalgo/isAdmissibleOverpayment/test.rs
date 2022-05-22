use super::isAdmissibleOverpayment::solution;

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test1(){
        assert_eq!(
            true,
            solution(
                vec![110.0, 95.0, 70.0], 
                vec![
                    String::from("10.0% higher than in-store"), 
                    String::from("5.0% lower than in-store"), 
                    String::from("Same as in-store")
                ], 
                5.0
            )
        );
    }

    #[test]
    fn test2(){
        assert_eq!(
            false,
            solution(
                vec![48.0, 165.0], 
                vec![
                    String::from("20.00% lower than in-store"), 
                    String::from("10.00% higher than in-store")
                ], 
                2.0
            )
        );
    }

    #[test]
    fn test3(){
        assert_eq!(
            true,
            solution(
                vec![24.42, 24.42, 24.2424, 85.23], 
                vec![
                    String::from("13.157% higher than in-store"), 
                    String::from("13.157% lower than in-store"), 
                    String::from("Same as in-store"), 
                    String::from("19.24% higher than in-store")
                ], 
                24.24
            )
        );
    }
    #[test]
    fn test4(){
        assert_eq!(
            true,
            solution(
                vec![220.0], 
                vec! [
                    String::from("120.0000% higher than in-store")
                ], 
                120.0
            )
        );
    }

    #[test]
    fn test5(){
        assert_eq!(
            true,
            solution(
                vec![40.0, 40.0, 40.0, 40.0], 
                vec![
                    String::from("10.0% higher than in-store"), 
                    String::from("10.0% lower than in-store"), 
                    String::from("10.0% higher than in-store"), 
                    String::from("10.0% lower than in-store")
                ], 
                0.0
            )
        );
    }
    #[test]
    fn test6(){
        assert_eq!(
            false,
            solution(
                vec![40.0, 40.0, 40.0, 40.0], 
                vec![
                    String::from("0.001% higher than in-store"), 
                    String::from("0.0% lower than in-store"), 
                    String::from("0.0% higher than in-store"), 
                    String::from("0.0% lower than in-store")
                ], 
                0.0
            )
        );
    }

    #[test]
    fn test7(){
        assert_eq!(
            true,
            solution(
                vec![110.0, 110.0, 110.0, 110.0, 110.0, 110.0, 110.0, 110.0, 110.0, 160.0], 
                vec![
                    String::from("10.0% higher than in-store"), 
                    String::from("10.0% higher than in-store"), 
                    String::from("10.0% higher than in-store"), 
                    String::from("10.0% higher than in-store"), 
                    String::from("10.0% higher than in-store"), 
                    String::from("10.0% higher than in-store"), 
                    String::from("10.0% higher than in-store"), 
                    String::from("10.0% higher than in-store"), 
                    String::from("10.0% higher than in-store"), 
                    String::from("60.0% higher than in-store")
                ], 
                150.0
            )
        );
    }
    #[test]
    fn test8(){
        assert_eq!(
            false,
            solution(
                vec![20.0], 
                vec![
                    String::from("100.0% higher than in-store"), 
                ], 
                9.0
            )
        );
    }

    #[test]
    fn test9(){
        assert_eq!(
            false,
            solution(
                vec![
                    35000.0, 35000.0
                ], 
                vec![
                    String::from("35000.0% higher than in-store"), 
                    String::from("10000.0% lower than in-store")
                ], 
                150.0
            )
        );
    }
    #[test]
    fn test10(){
        assert_eq!(
            true,
            solution(
                vec![20.0, 20.0], 
                vec![
                    String::from("20.0% higher than in-store"), 
                    String::from("20.0% lower than in-store")
                ], 
                0.0
            )
        );
    }
}