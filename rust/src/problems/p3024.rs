pub struct Solution;

impl Solution {
    pub fn triangle_type(mut nums: Vec<i32>) -> String {
        if nums[0] == nums[1] && nums[1] == nums[2] {
            return "equilateral".to_string();
        }
        nums.sort_unstable();
        if nums[0] + nums[1] <= nums[2] {
            "none".to_string()
        } else if nums[1] == nums[0] || nums[1] == nums[2] {
            "isosceles".to_string()
        } else {
            "scalene".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("equilateral", Solution::triangle_type(vec![3, 3, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!("scalene", Solution::triangle_type(vec![3, 4, 5]));
    }
}
