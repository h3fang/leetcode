pub struct Solution;

impl Solution {
    pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
        (max_weight / w).min(n * n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::max_containers(2, 3, 15));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::max_containers(3, 5, 20));
    }
}
