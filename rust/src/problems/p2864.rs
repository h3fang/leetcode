pub struct Solution;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let n = s.len();
        let ones = s.chars().filter(|&c| c == '1').count();
        format!("{}{}1", "1".repeat(ones - 1), "0".repeat(n - ones))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "001",
            Solution::maximum_odd_binary_number("010".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "1001",
            Solution::maximum_odd_binary_number("0101".to_string())
        );
    }
}
