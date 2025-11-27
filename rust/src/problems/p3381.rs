pub struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let (n, k) = (nums.len(), k as usize);
        let mut pre = Vec::with_capacity(n + 1);
        pre.push(0);
        for x in nums {
            pre.push(pre.last().unwrap() + x as i64);
        }
        let mut min = vec![i64::MAX / 2; k];
        let mut ans = i64::MIN;
        for (j, s) in pre.into_iter().enumerate() {
            let i = j % k;
            ans = ans.max(s - min[i]);
            min[i] = min[i].min(s);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::max_subarray_sum(vec![1, 2], 1));
    }

    #[test]
    fn case2() {
        assert_eq!(-10, Solution::max_subarray_sum(vec![-1, -2, -3, -4, -5], 4));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::max_subarray_sum(vec![-5, 1, 2, -3, 4], 2));
    }
}
