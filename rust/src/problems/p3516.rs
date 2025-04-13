pub struct Solution;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        match x.abs_diff(z).cmp(&y.abs_diff(z)) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::find_closest(2, 7, 4));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::find_closest(2, 5, 6));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::find_closest(1, 5, 3));
    }
}
