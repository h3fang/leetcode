pub struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let n = salary.len() as f64;
        let (min, max, sum) = salary
            .into_iter()
            .fold((i32::MAX, i32::MIN, 0), |(min, max, sum), s| {
                (min.min(s), max.max(s), sum + s)
            });
        (sum - min - max) as f64 / (n - 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "a = {:.5}, b = {:.5}", a, b);
    }

    #[test]
    fn case1() {
        assert_close(2500.0, Solution::average(vec![4000, 3000, 1000, 2000]));
    }

    #[test]
    fn case2() {
        assert_close(2000.0, Solution::average(vec![1000, 2000, 3000]));
    }
}
