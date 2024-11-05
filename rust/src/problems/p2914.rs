pub struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes()
            .chunks(2)
            .map(|w| i32::from(w[0] != w[1]))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::min_changes("1001".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::min_changes("10".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::min_changes("0000".to_string()));
    }
}
