pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut m: HashMap<i32, i32> = HashMap::new();
        let mut left = vec![0; n];
        for i in -(n as i32)..n as i32 {
            if i >= 0 {
                left[i as usize] = m[&nums[i as usize]];
            }
            let j = (i + n as i32) as usize;
            m.insert(nums[j % n], i);
        }

        m.clear();
        let mut right = vec![0; n];
        for i in (0..n * 2).rev() {
            if i < n {
                right[i] = m[&nums[i]];
            }
            m.insert(nums[i % n], i as i32);
        }
        queries
            .into_iter()
            .map(|q| {
                let r = (q - left[q as usize]).min(right[q as usize] - q);
                if r == n as i32 {
                    -1
                } else {
                    r
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![2, -1, 3],
            Solution::solve_queries(vec![1, 3, 1, 4, 1, 3, 2], vec![0, 3, 5])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![-1, -1, -1, -1],
            Solution::solve_queries(vec![1, 2, 3, 4], vec![0, 1, 2, 3])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            vec![-1, -1, 2, -1, -1, 2],
            Solution::solve_queries(
                vec![15, 1, 10, 1, 20, 4, 6, 14, 4, 9, 4, 18],
                vec![0, 2, 10, 6, 11, 8]
            )
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            vec![-1, 1, 1, 2, -1],
            Solution::solve_queries(vec![14, 14, 4, 2, 19, 19, 14, 19, 14], vec![2, 4, 8, 6, 3])
        );
    }
}
