pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut furthest = 0;
        let mut next_furthest = 0;
        let mut result = 0;
        for (i, &x) in nums[..n - 1].iter().enumerate() {
            next_furthest = next_furthest.max(i + x as usize);
            if i == furthest {
                furthest = next_furthest;
                result += 1;
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
        assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::jump(vec![2, 3, 0, 1, 4]));
    }
}
