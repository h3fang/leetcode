pub struct Solution;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

impl Solution {
    pub fn is_good_array(nums: Vec<i32>) -> bool {
        nums.into_iter().reduce(gcd).unwrap() == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_good_array(vec![12, 5, 7, 23]));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_good_array(vec![29, 6, 10]));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_good_array(vec![6, 3]));
    }
}
