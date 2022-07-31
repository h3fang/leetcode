pub struct Solution;

impl Solution {
    pub fn minimum_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut result = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                continue;
            }
            let v = nums[i];
            for n in &mut nums[i..] {
                *n -= v;
            }
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::minimum_operations(vec![1, 5, 0, 3, 5]));
    }
}
