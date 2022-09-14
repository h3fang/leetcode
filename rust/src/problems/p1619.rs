pub struct Solution;

impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        arr.sort_unstable();
        let n = arr.len() / 20;
        arr[n..arr.len() - n].iter().sum::<i32>() as f64 / (arr.len() - 2 * n) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = Solution::trim_mean(vec![
            1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3,
        ]);
        assert!((2.0 - result).abs() < 1e-5);
    }

    #[test]
    fn case2() {
        let result = Solution::trim_mean(vec![
            6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5, 10,
            8, 6, 6, 1, 0, 6, 10, 8, 2, 3, 4,
        ]);
        assert!((4.77778 - result).abs() < 1e-5);
    }
}
