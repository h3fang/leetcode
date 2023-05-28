pub struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        fn merge(f: &[i32], g: &[i32], mut k: i32) -> Vec<i32> {
            if g.len() > f.len() {
                return merge(g, f, k);
            }
            let mut q = BinaryHeap::new();
            for (i, &g) in g.iter().enumerate() {
                q.push((Reverse(f[0] + g), 0, i));
            }
            let mut result = Vec::with_capacity(f.len());
            while k > 0 && !q.is_empty() {
                let (Reverse(sum), i, j) = q.pop().unwrap();
                result.push(sum);
                if i + 1 < f.len() {
                    q.push((Reverse(f[i + 1] + g[j]), i + 1, j));
                }
                k -= 1;
            }
            result
        }
        let mut prev = mat[0].to_vec();
        for g in &mat[1..] {
            prev = merge(&prev, g, k);
        }
        prev[k as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = [[1, 3, 11], [2, 4, 6]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(7, Solution::kth_smallest(mat, 5));
    }

    #[test]
    fn case2() {
        let mat = [[1, 3, 11], [2, 4, 6]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(17, Solution::kth_smallest(mat, 9));
    }

    #[test]
    fn case3() {
        let mat = [[1, 10, 10], [1, 4, 5], [2, 3, 6]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(9, Solution::kth_smallest(mat, 7));
    }

    #[test]
    fn case4() {
        let mat = [[1, 1, 10], [2, 2, 9]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(12, Solution::kth_smallest(mat, 7));
    }
}
