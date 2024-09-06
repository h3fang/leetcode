pub struct Solution;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut count = [0; 26];
        for c in s.as_bytes() {
            count[(*c - b'a') as usize] += 1;
        }

        let odd = count.iter().filter(|&&c| c % 2 == 1).count() as i32;

        k >= odd && k <= s.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::can_construct("cr".to_string(), 7));
    }

    #[test]
    fn case2() {
        assert!(Solution::can_construct("qlkzenwmmnpkopu".to_string(), 15));
    }
}
