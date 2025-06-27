pub struct Solution;

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        vec![celsius + 273.15, celsius * 1.8 + 32.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "a = {a:.5}, b = {b:.5}");
    }

    #[test]
    fn case1() {
        let expected = vec![309.65000, 97.70000];
        let result = Solution::convert_temperature(36.50);
        assert_eq!(2, result.len());
        result
            .into_iter()
            .zip(expected)
            .for_each(|(r, e)| assert_close(r, e));
    }
}
