pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn sequence_reconstruction(nums: Vec<i32>, sequences: Vec<Vec<i32>>) -> bool {
        let n = nums.len();
        let mut q = VecDeque::new();
        let mut in_deg = vec![0; n];
        let mut g = vec![HashSet::new(); n];
        for seq in &sequences {
            for w in seq.windows(2) {
                let a = w[0] - 1;
                let b = w[1] - 1;
                if !g[a as usize].contains(&b) {
                    g[a as usize].insert(b);
                    in_deg[b as usize] += 1;
                }
            }
        }
        for (i, &d) in in_deg.iter().enumerate() {
            if d == 0 {
                q.push_back(i);
            }
        }
        while !q.is_empty() {
            if q.len() > 1 {
                return false;
            }
            let i = q.pop_front().unwrap();
            for &next in &g[i] {
                in_deg[next as usize] -= 1;
                if in_deg[next as usize] == 0 {
                    q.push_back(next as usize);
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 2, 3];
        let sequences = [[1, 2], [1, 3]];
        let sequences = sequences.iter().map(|s| s.to_vec()).collect();
        assert_eq!(false, Solution::sequence_reconstruction(nums, sequences));
    }

    #[test]
    fn case2() {
        let nums = vec![1, 2, 3];
        let sequences = [[1, 2]];
        let sequences = sequences.iter().map(|s| s.to_vec()).collect();
        assert_eq!(false, Solution::sequence_reconstruction(nums, sequences));
    }

    #[test]
    fn case3() {
        let nums = vec![1, 2, 3];
        let sequences = [[1, 2], [1, 3], [2, 3]];
        let sequences = sequences.iter().map(|s| s.to_vec()).collect();
        assert_eq!(true, Solution::sequence_reconstruction(nums, sequences));
    }
}
