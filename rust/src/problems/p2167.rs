pub struct Solution;

impl Solution {
    pub fn minimum_time(s: String) -> i32 {
        let n = s.len();
        let mut result = n;
        let mut pre = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                pre = (pre + 2).min(i + 1);
            }
            result = result.min(pre + n - i - 1);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::minimum_time("1100101".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::minimum_time("0010".to_string()));
    }
}
