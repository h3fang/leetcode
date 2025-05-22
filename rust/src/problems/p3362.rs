pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        let n = queries.len();
        queries.sort_unstable_by_key(|e| e[0]);
        let mut diff = vec![0; nums.len() + 1];
        let (mut sum, mut j) = (0, 0);
        let mut q = BinaryHeap::with_capacity(n);
        for (i, &x) in nums.iter().enumerate() {
            sum += diff[i];
            while j < n && queries[j][0] <= i as i32 {
                q.push(queries[j][1]);
                j += 1;
            }
            while x > sum && q.peek().is_some_and(|&e| e >= i as i32) {
                sum += 1;
                let k = q.pop().unwrap();
                diff[k as usize + 1] -= 1;
            }
            if x > sum {
                return -1;
            }
        }
        q.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![2, 0, 2];
        let queries = [[0, 2], [0, 2], [1, 1]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(1, Solution::max_removal(nums, queries));
    }

    #[test]
    fn case2() {
        let nums = vec![1, 1, 1, 1];
        let queries = [[1, 3], [0, 2], [1, 3], [1, 2]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(2, Solution::max_removal(nums, queries));
    }

    #[test]
    fn case3() {
        let nums = vec![1, 2, 3, 4];
        let queries = [[0, 3]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(-1, Solution::max_removal(nums, queries));
    }
}
