pub struct Solution;

impl Solution {
    pub fn kth_grammar(_n: i32, k: i32) -> i32 {
        (k - 1).count_ones() as i32 & 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::kth_grammar(1, 1));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::kth_grammar(2, 1));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::kth_grammar(2, 2));
    }
}
