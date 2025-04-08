pub struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut seen = [false; 101];
        for (i, x) in nums.into_iter().enumerate().rev() {
            if seen[x as usize] {
                return (i + 1).div_ceil(3) as i32;
            }
            seen[x as usize] = true;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::minimum_operations(vec![4, 5, 6, 4, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::minimum_operations(vec![6, 7, 8, 9]));
    }
}
