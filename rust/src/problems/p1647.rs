pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut f = [0; 26];
        for &b in s.as_bytes() {
            let i = (b - b'a') as usize;
            f[i] += 1;
        }
        let mut q = BinaryHeap::new();
        for e in f {
            if e > 0 {
                q.push(e);
            }
        }
        let mut result = 0;
        while let Some(f) = q.pop() {
            if let Some(&f1) = q.peek()
                && f == f1
            {
                if f > 1 {
                    q.push(f - 1);
                }
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::min_deletions("aab".into()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_deletions("aaabbbcc".into()));
    }
}
