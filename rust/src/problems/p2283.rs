pub struct Solution;

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut count = [0; 10];
        for &b in num.as_bytes() {
            count[(b - b'0') as usize] += 1;
        }
        num.as_bytes()
            .iter()
            .enumerate()
            .all(|(i, b)| count[i] == b - b'0')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::digit_count("1210".into()));
    }

    #[test]
    fn case2() {
        assert!(!Solution::digit_count("030".into()));
    }
}
