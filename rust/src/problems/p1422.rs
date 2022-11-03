pub struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s = s.as_bytes();
        let mut score =
            i32::from(s[0] == b'0') + s[1..].iter().filter(|&&b| b == b'1').count() as i32;
        let mut result = score;
        for &b in &s[1..s.len() - 1] {
            if b == b'1' {
                score -= 1;
            } else {
                score += 1;
            }
            result = result.max(score);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::max_score("011101".into()));
    }
}
