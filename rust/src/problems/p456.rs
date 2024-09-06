pub struct Solution;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 3 {
            return false;
        }
        let mut s = vec![];
        let mut max_2 = i32::MIN;
        for &n in nums.iter().rev() {
            if n < max_2 {
                return true;
            }
            while !s.is_empty() && *s.last().unwrap() < n {
                max_2 = s.pop().unwrap();
            }
            if n > max_2 {
                s.push(n);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::find132pattern(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert!(Solution::find132pattern(vec![3, 1, 4, 2]));
    }

    #[test]
    fn case3() {
        assert!(Solution::find132pattern(vec![3, 5, 0, 3, 4]));
    }
}
