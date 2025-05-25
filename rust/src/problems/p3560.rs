pub struct Solution;

impl Solution {
    pub fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
        let b = n.max(m);
        (b - k).max(0) as i64 * k as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::min_cutting_cost(6, 5, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_cutting_cost(4, 4, 6));
    }
}
