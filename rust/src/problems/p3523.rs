pub struct Solution;

impl Solution {
    pub fn maximum_possible_size(nums: Vec<i32>) -> i32 {
        let (mut max, mut ans) = (0, 0);
        for x in nums {
            if x >= max {
                ans += 1;
                max = x;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::maximum_possible_size(vec![4, 2, 5, 3, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::maximum_possible_size(vec![1, 2, 3]));
    }
}
