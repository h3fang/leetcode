pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let n = s.len();
        let d = dictionary.into_iter().collect::<HashSet<_>>();
        let mut f = vec![0; n + 1];
        for i in 0..n {
            f[i + 1] = f[i] + 1;
            for j in 0..=i {
                if d.contains(&s[j..i + 1]) {
                    f[i + 1] = f[i + 1].min(f[j]);
                }
            }
        }
        f[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "leetscode".to_string();
        let dictionary = ["leet", "code", "leetcode"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(1, Solution::min_extra_char(s, dictionary))
    }

    #[test]
    fn case2() {
        let s = "sayhelloworld".to_string();
        let dictionary = ["hello", "world"].iter().map(|s| s.to_string()).collect();
        assert_eq!(3, Solution::min_extra_char(s, dictionary))
    }
}
