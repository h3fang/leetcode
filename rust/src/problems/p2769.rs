pub struct Solution;

impl Solution {
    pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
        num + t * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::the_maximum_achievable_x(4, 1));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::the_maximum_achievable_x(3, 2));
    }
}
