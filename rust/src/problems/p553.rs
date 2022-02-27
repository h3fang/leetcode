pub struct Solution;

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        if nums.len() <= 2 {
            let mut s = nums[0].to_string();
            for n in &nums[1..] {
                s.push('/');
                s.push_str(&n.to_string());
            }
            s
        } else {
            let mut s = format!("{}/({}", nums[0], nums[1]);
            for n in &nums[2..] {
                s.push('/');
                s.push_str(&n.to_string());
            }
            s.push(')');
            s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "1000/(100/10/2)",
            Solution::optimal_division(vec![1000, 100, 10, 2])
        );
    }
}
