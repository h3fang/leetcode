pub struct Solution;

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        for &n in &nums[1..] {
            match n.abs().cmp(&result.abs()) {
                std::cmp::Ordering::Less => result = n,
                std::cmp::Ordering::Equal => result = result.max(n),
                std::cmp::Ordering::Greater => {}
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
        assert_eq!(1, Solution::find_closest_number(vec![-4, -2, 1, 4, 8]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::find_closest_number(vec![2, -1, 1]));
    }
}
