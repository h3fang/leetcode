pub struct Solution;

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        (left.into_iter().max().unwrap_or(0)).max(n - right.into_iter().min().unwrap_or(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::get_last_moment(4, vec![4, 3], vec![0, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            7,
            Solution::get_last_moment(7, vec![], vec![0, 1, 2, 3, 4, 5, 6, 7])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            7,
            Solution::get_last_moment(7, vec![0, 1, 2, 3, 4, 5, 6, 7], vec![])
        );
    }
}
