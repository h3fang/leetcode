pub struct Solution;

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut left = 1;
        let mut right = nums.len() as i32;
        while left <= right {
            let m = left + (right - left) / 2;
            let c = nums.iter().filter(|n| **n >= m).count() as i32;
            match m.cmp(&c) {
                std::cmp::Ordering::Less => left = m + 1,
                std::cmp::Ordering::Equal => return m,
                std::cmp::Ordering::Greater => right = m - 1,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::special_array(vec![3, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::special_array(vec![0, 0]));
    }
}
