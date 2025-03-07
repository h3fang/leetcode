pub struct Solution;

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        for (i, &x) in nums.iter().enumerate() {
            for (j, &y) in nums.iter().enumerate().skip(i + index_difference as usize) {
                if (x - y).abs() >= value_difference {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(nums: Vec<i32>, index_difference: i32, value_difference: i32) {
        let n = nums.len() as i32;
        let r = Solution::find_indices(nums.clone(), index_difference, value_difference);
        assert!(r.len() == 2);
        assert!(r[0] >= 0 && r[0] < n);
        assert!(r[1] >= 0 && r[1] < n);
        let a = nums[r[0] as usize];
        let b = nums[r[1] as usize];
        assert!((r[0] - r[1]).abs() >= index_difference);
        assert!((a - b).abs() >= value_difference);
    }

    #[test]
    fn case1() {
        check(vec![5, 1, 4, 1], 2, 4);
    }

    #[test]
    fn case2() {
        check(vec![2, 1], 0, 0);
    }

    #[test]
    fn case3() {
        assert_eq!(vec![-1, -1], Solution::find_indices(vec![1, 2, 3], 2, 4));
    }
}
