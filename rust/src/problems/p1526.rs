pub struct Solution;

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        target[0] + target.windows(2).map(|w| (w[1] - w[0]).max(0)).sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::min_number_operations(vec![1, 2, 3, 2, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::min_number_operations(vec![3, 1, 1, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(7, Solution::min_number_operations(vec![3, 1, 5, 4, 2]));
    }

    #[test]
    fn case4() {
        assert_eq!(1, Solution::min_number_operations(vec![1, 1, 1, 1]));
    }
}
