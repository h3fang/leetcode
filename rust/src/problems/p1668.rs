pub struct Solution;

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        for i in 1.. {
            if !sequence.contains(&word.repeat(i)) {
                return i as i32 - 1;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::max_repeating("ababc".to_string(), "ab".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::max_repeating("ababc".to_string(), "ba".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            0,
            Solution::max_repeating("ababc".to_string(), "ac".to_string())
        );
    }
}
