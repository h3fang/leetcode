pub struct Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut freq = [0i32; 26];
        for (a, b) in s.as_bytes().iter().zip(t.as_bytes()) {
            freq[(a - b'a') as usize] -= 1;
            freq[(b - b'a') as usize] += 1;
        }

        freq.into_iter().filter(|&e| e > 0).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_steps("bab".to_string(), "aba".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(
            5,
            Solution::min_steps("leetcode".to_string(), "practice".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            0,
            Solution::min_steps("anagram".to_string(), "mangaar".to_string())
        );
    }
}
