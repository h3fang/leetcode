pub struct Solution;

impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        nums.push(-1);
        let mut result = 0;
        for i in 0..nums.len() {
            while nums[i] != -1 && nums[i] != i as i32 {
                let j = nums[i];
                nums.swap(i, j as usize);
            }
            if nums[i] == -1 {
                result = i;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::missing_number(vec![3, 0, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::missing_number(vec![0, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(8, Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]));
    }
}
