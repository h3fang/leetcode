use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut freq = HashMap::new();
        for n in nums {
            *freq.entry(n).or_insert(0) += 1;
        }
        let mut q = BinaryHeap::new();
        for (n, f) in freq {
            q.push(Reverse((f, n)));
            if q.len() > k {
                q.pop();
            }
        }
        q.iter().map(|v| v.0 .1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut expected = vec![1, 2];
        expected.sort_unstable();
        let mut result = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let mut expected = vec![1];
        expected.sort_unstable();
        let mut result = Solution::top_k_frequent(vec![1], 1);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case3() {
        let mut expected = vec![-1, 2];
        expected.sort_unstable();
        let mut result = Solution::top_k_frequent(vec![4, 1, -1, 2, -1, 2, 3], 2);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
