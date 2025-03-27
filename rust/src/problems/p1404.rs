pub struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let (s, n) = (s.as_bytes(), s.len() as i32);
        let i = s.iter().rposition(|x| *x == b'1').unwrap();
        if i == 0 {
            return n - 1;
        }
        let ans = s[..=i].iter().filter(|x| **x == b'0').count();
        ans as i32 + n + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::num_steps("1101".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::num_steps("10".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::num_steps("1".to_string()));
    }
}
