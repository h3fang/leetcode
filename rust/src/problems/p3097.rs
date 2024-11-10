pub struct Solution;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = nums.len() + 1;
        let mut ors = [(0, 0); 32];
        let mut m = 0;
        for (i, &x) in nums.iter().enumerate() {
            ors[m] = (0, i);
            m += 1;
            let mut j = 0;
            for p in 0..m {
                ors[p].0 |= x;
                if ors[p].0 >= k {
                    result = result.min(i - ors[p].1 + 1);
                }
                if ors[j].0 == ors[p].0 {
                    ors[j].1 = ors[p].1;
                } else {
                    j += 1;
                    ors[j] = ors[p];
                }
            }
            m = j + 1;
        }
        if result == nums.len() + 1 {
            -1
        } else {
            result as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::minimum_subarray_length(vec![1, 2, 3], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::minimum_subarray_length(vec![2, 1, 8], 10));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::minimum_subarray_length(vec![1, 2], 0));
    }
}
