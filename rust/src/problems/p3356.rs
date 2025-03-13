pub struct Solution;

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let m = nums.len();
        let n = queries.len();
        let mut diff = vec![0; m + 1];
        let (mut sum, mut j) = (0, 0);
        for (i, x) in nums.into_iter().enumerate() {
            sum += diff[i];
            while x > sum && j < n {
                let q = &queries[j];
                diff[q[0] as usize] += q[2];
                diff[q[1] as usize + 1] -= q[2];
                if q[0] <= i as i32 && q[1] >= i as i32 {
                    sum += q[2];
                }
                j += 1;
            }
            if x > sum {
                return -1;
            }
        }
        j as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![2, 0, 2];
        let queries = [[0, 2, 1], [0, 2, 1], [1, 1, 3]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(2, Solution::min_zero_array(nums, queries));
    }

    #[test]
    fn case2() {
        let nums = vec![4, 3, 2, 1];
        let queries = [[1, 3, 2], [0, 2, 1]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(-1, Solution::min_zero_array(nums, queries));
    }
}
