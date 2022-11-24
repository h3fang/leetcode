pub struct Solution;

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut last1 = -1;
        let mut last2 = -1;
        let mut result = 0;
        for (i, &n) in nums.iter().enumerate() {
            if (left..=right).contains(&n) {
                last1 = i as i32;
            } else if n > right {
                last2 = i as i32;
                last1 = -1;
            }
            if last1 != -1 {
                result += last1 - last2;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::num_subarray_bounded_max(vec![2, 1, 4, 3], 2, 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            7,
            Solution::num_subarray_bounded_max(vec![2, 9, 2, 5, 6], 2, 8)
        );
    }
}
