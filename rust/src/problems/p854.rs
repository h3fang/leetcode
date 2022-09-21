pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn k_similarity(mut s1: String, mut s2: String) -> i32 {
        let s1 = unsafe { s1.as_bytes_mut() };
        let s2 = unsafe { s2.as_bytes_mut() };
        let n = s1.len();
        let mut q = VecDeque::new();
        q.push_back((s1.to_vec(), 0));
        let mut seen = HashSet::new();
        seen.insert(s1.to_vec());
        for steps in 0.. {
            let size = q.len();
            for _ in 0..size {
                let (mut s, mut p) = q.pop_front().unwrap();
                while p < n && s[p] == s2[p] {
                    p += 1;
                }
                if p == n {
                    return steps;
                }
                for j in p + 1..n {
                    if s[j] != s2[j] && s[j] == s2[p] {
                        s.swap(j, p);
                        if !seen.contains(&s) {
                            q.push_back((s.to_vec(), p + 1));
                            seen.insert(s.to_vec());
                        }
                        s.swap(j, p);
                    }
                }
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
            1,
            Solution::k_similarity("ab".to_string(), "ba".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::k_similarity("abc".to_string(), "bca".to_string())
        );
    }
}
