pub struct Solution;

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        (1..=n)
            .filter(|&x| x % 3 == 0 || x % 5 == 0 || x % 7 == 0)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(21, Solution::sum_of_multiples(7));
    }

    #[test]
    fn case2() {
        assert_eq!(40, Solution::sum_of_multiples(10));
    }

    #[test]
    fn case3() {
        assert_eq!(30, Solution::sum_of_multiples(9));
    }
}
