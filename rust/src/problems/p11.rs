pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut result = 0;
        while left < right {
            result = result.max(height[left].min(height[right]) * (right - left) as i32);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
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
        assert_eq!(1, Solution::max_area(vec![1, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(16, Solution::max_area(vec![4, 3, 2, 1, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::max_area(vec![1, 2, 1]));
    }
}
