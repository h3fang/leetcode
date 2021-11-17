pub struct Solution;

impl Solution {
    pub fn construct_arr(a: Vec<i32>) -> Vec<i32> {
        if a.is_empty() {
            return a;
        }
        let n = a.len();
        let mut result = vec![0; n];
        result[0] = 1;
        for i in 1..n {
            result[i] = result[i - 1] * a[i - 1];
        }
        let mut right = 1;
        for i in (0..n - 1).rev() {
            right *= a[i + 1];
            result[i] *= right;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let a = vec![1, 2, 3, 4, 5];
        let expected = vec![120, 60, 40, 30, 24];
        assert_eq!(expected, Solution::construct_arr(a));
    }
}
