pub struct Solution;

use std::collections::VecDeque;

fn is_seq(next: &[[i32; 26]], seq: &[u8], k: i32) -> bool {
    let mut i = -1;
    for _ in 0..k {
        for &b in seq {
            i = next[(i + 1) as usize][(b - b'a') as usize];
            if i == i32::MAX {
                return false;
            }
        }
    }
    true
}

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let n = s.len();
        let mut f = [0; 26];
        for b in s.as_bytes() {
            f[(b - b'a') as usize] += 1;
        }
        let mut chars = Vec::with_capacity(n);
        for (i, &f) in f.iter().enumerate() {
            if f >= k {
                chars.push(i as u8 + b'a');
            }
        }

        let mut next = vec![[i32::MAX; 26]; n + 1];
        for (i, &b) in s.as_bytes().iter().enumerate().rev() {
            next[i] = next[i + 1];
            next[i][(b - b'a') as usize] = i as i32;
        }

        let mut ans = Vec::with_capacity(n);
        let mut q = VecDeque::with_capacity(1024);
        q.push_back(vec![]);
        while let Some(seq) = q.pop_front() {
            for &b in &chars {
                let mut seq1 = seq.clone();
                seq1.push(b);

                if is_seq(&next, &seq1, k) {
                    ans = seq1.clone();
                    q.push_back(seq1);
                }
            }
        }
        unsafe { String::from_utf8_unchecked(ans) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "let",
            Solution::longest_subsequence_repeated_k("letsleetcode".to_string(), 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "b",
            Solution::longest_subsequence_repeated_k("bb".to_string(), 2)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "",
            Solution::longest_subsequence_repeated_k("ab".to_string(), 2)
        );
    }
}
