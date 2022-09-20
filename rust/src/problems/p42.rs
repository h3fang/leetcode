pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut l_max = 0;
        let mut r_max = 0;
        let mut l = 0;
        let mut r = height.len() - 1;
        while l < r {
            if height[l] < height[r] {
                if height[l] > l_max {
                    l_max = height[l];
                } else {
                    result += l_max - height[l];
                }
                l += 1;
            } else {
                if height[r] > r_max {
                    r_max = height[r];
                } else {
                    result += r_max - height[r];
                }
                r -= 1;
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
        assert_eq!(6, Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    }
}
