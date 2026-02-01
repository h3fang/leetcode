pub struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (i32::MAX, i32::MAX);
        for &x in &nums[1..] {
            if x < a {
                (a, b) = (x, a);
            } else if x < b {
                b = x;
            }
        }
        nums[0] + a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::minimum_cost(vec![1, 2, 3, 12]));
    }

    #[test]
    fn case2() {
        assert_eq!(12, Solution::minimum_cost(vec![5, 4, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(12, Solution::minimum_cost(vec![10, 3, 1, 1]));
    }
}
