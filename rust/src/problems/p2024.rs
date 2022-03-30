pub struct Solution;

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let mut count = [0; 2];
        let mut left = 0;
        let mut max = 0;
        let key = answer_key.as_bytes();
        for (right, &c) in key.iter().enumerate() {
            if c == b'T' {
                count[0] += 1;
                max = max.max(count[0]);
            } else {
                count[1] += 1;
                max = max.max(count[1]);
            }
            if right - left + 1 > max + k as usize {
                let c = key[left];
                if c == b'T' {
                    count[0] -= 1;
                } else {
                    count[1] -= 1;
                }
                left += 1;
            }
        }
        (answer_key.len() - left) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::max_consecutive_answers("TTFF".to_string(), 2));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::max_consecutive_answers("TFFT".to_string(), 1));
    }

    #[test]
    fn case3() {
        assert_eq!(
            5,
            Solution::max_consecutive_answers("TTFTTFTT".to_string(), 1)
        );
    }
}
