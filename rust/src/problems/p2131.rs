pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut m = HashMap::new();
        for w in &words {
            let w = w.as_bytes();
            let w = [w[0], w[1]];
            *m.entry(w).or_insert(0) += 1;
        }
        let mut pairs = 0;
        let mut center = false;
        for (k, &v) in &m {
            if k[0] > k[1] {
                continue;
            }
            if k[0] != k[1] {
                let p = m.get(&[k[1], k[0]]).cloned().unwrap_or(0);
                pairs += p.min(v);
            } else {
                if v % 2 == 1 && !center {
                    center = true;
                }
                pairs += v / 2;
            }
        }
        4 * pairs + if center { 2 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["lc", "cl", "gg"].iter().map(|w| w.to_string()).collect();
        assert_eq!(6, Solution::longest_palindrome(words));
    }

    #[test]
    fn case2() {
        let words = ["ab", "ty", "yt", "lc", "cl", "ab"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(8, Solution::longest_palindrome(words));
    }

    #[test]
    fn case3() {
        let words = ["cc", "ll", "xx"].iter().map(|w| w.to_string()).collect();
        assert_eq!(2, Solution::longest_palindrome(words));
    }
}
