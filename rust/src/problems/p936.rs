pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let s = stamp.as_bytes();
        let t = target.as_bytes();
        let m = s.len();
        let n = t.len();
        let mut done = vec![false; n];
        let mut arr = vec![];
        let mut q = VecDeque::new();
        let mut result = vec![];
        for i in 0..n - m + 1 {
            let mut todo = HashSet::new();
            for j in 0..m {
                if s[j] != t[i + j] {
                    todo.insert(i + j);
                }
            }
            if todo.is_empty() {
                result.push(i as i32);
                for (j, d) in done.iter_mut().enumerate().skip(i).take(m) {
                    if !*d {
                        q.push_back(j);
                        *d = true;
                    }
                }
            }
            arr.push(todo);
        }
        while let Some(i) = q.pop_front() {
            let lb = 0.max(i as i32 + 1 - m as i32) as usize;
            let ub = i.min(n - m);
            for (j, a) in arr.iter_mut().enumerate().take(ub + 1).skip(lb) {
                if a.contains(&i) {
                    a.remove(&i);
                    if a.is_empty() {
                        result.push(j as i32);
                        for (k, d) in done.iter_mut().enumerate().skip(j).take(m) {
                            if !*d {
                                q.push_back(k);
                                *d = true;
                            }
                        }
                    }
                }
            }
        }
        if !done.iter().all(|d| *d) {
            return vec![];
        }
        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_valid(s: &str, t: &str, ops: &[i32]) {
        println!("{:?}", ops);
        let s = s.as_bytes();
        let mut bytes = vec![0; t.len()];
        for &i in ops {
            let i = i as usize;
            bytes[i..i + s.len()].copy_from_slice(s);
        }
        assert_eq!(t.as_bytes(), bytes);
    }

    #[test]
    fn case1() {
        let s = "abc".to_string();
        let t = "ababc".to_string();
        assert_valid(
            &s,
            &t,
            &Solution::moves_to_stamp(s.to_string(), t.to_string()),
        );
    }

    #[test]
    fn case2() {
        let s = "abca".to_string();
        let t = "aabcaca".to_string();
        assert_valid(
            &s,
            &t,
            &Solution::moves_to_stamp(s.to_string(), t.to_string()),
        );
    }
}
