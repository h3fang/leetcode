pub struct Solution;

impl Solution {
    pub fn maximum_groups(grades: Vec<i32>) -> i32 {
        fn is_valid(n: usize, k: usize) -> bool {
            let c = (1 + k) * k / 2;
            c <= n
        }
        let n = grades.len();
        let mut left = 1;
        let mut right = n;
        let mut result = 1;
        while left <= right {
            let k = (left + right) / 2;
            if is_valid(n, k) {
                result = result.max(k);
                left = k + 1;
            } else {
                right = k - 1;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::maximum_groups(vec![10, 6, 12, 7, 3, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::maximum_groups(vec![8, 8]));
    }
}
