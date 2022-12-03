pub struct Solution;

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut seen = [false; 10];
        for &b in s.as_bytes() {
            if b.is_ascii_digit() {
                let d = b - b'0';
                seen[d as usize] = true;
            }
        }
        seen.iter()
            .enumerate()
            .rev()
            .filter(|&(_, &s)| s)
            .nth(1)
            .map(|e| e.0 as i32)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::second_highest("dfa12321afd".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::second_highest("abc1111".to_string()));
    }
}
