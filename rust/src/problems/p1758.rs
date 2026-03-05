pub struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let k: usize = s
            .bytes()
            .enumerate()
            .map(|(i, b)| (i % 2).abs_diff((b - b'0') as usize))
            .sum();
        k.min(s.len() - k) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_operations("0100".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_operations("10".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::min_operations("1111".to_string()));
    }
}
