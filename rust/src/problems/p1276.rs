pub struct Solution;

impl Solution {
    pub fn num_of_burgers(a: i32, b: i32) -> Vec<i32> {
        let (x, y) = (a / 2 - b, 2 * b - a / 2);
        if a % 2 != 0 || x < 0 || y < 0 {
            return vec![];
        }
        vec![x, y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![1, 6], Solution::num_of_burgers(16, 7));
    }

    #[test]
    fn case2() {
        assert!(Solution::num_of_burgers(17, 4).is_empty());
    }

    #[test]
    fn case3() {
        assert!(Solution::num_of_burgers(4, 17).is_empty());
    }

    #[test]
    fn case4() {
        assert_eq!(vec![0, 0], Solution::num_of_burgers(0, 0));
    }

    #[test]
    fn case5() {
        assert_eq!(vec![0, 1], Solution::num_of_burgers(2, 1));
    }
}
