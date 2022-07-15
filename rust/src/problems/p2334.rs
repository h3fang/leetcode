pub struct Solution;

impl Solution {
    pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
        let n = nums.len();
        let mut left = vec![-1; n];
        let mut s = Vec::with_capacity(n);
        for (i, &e) in nums.iter().enumerate() {
            while !s.is_empty() && nums[*s.last().unwrap()] >= e {
                s.pop();
            }
            if let Some(&j) = s.last() {
                left[i] = j as i32;
            }
            s.push(i);
        }

        let mut right = vec![n as i32; n];
        s.clear();
        for (i, &e) in nums.iter().enumerate().rev() {
            while !s.is_empty() && nums[*s.last().unwrap()] >= e {
                s.pop();
            }
            if let Some(&j) = s.last() {
                right[i] = j as i32;
            }
            s.push(i);
        }
        for ((&e, &l), &r) in nums.iter().zip(&left).zip(&right) {
            let k = r - l - 1;
            if e > threshold / k {
                return k;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::valid_subarray_size(vec![1, 3, 4, 3, 1], 6));
    }

    #[test]
    fn case2() {
        let r = Solution::valid_subarray_size(vec![6, 5, 6, 5, 8], 7);
        assert!(&[1, 2, 3, 4, 5].contains(&r));
    }
}
