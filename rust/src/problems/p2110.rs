pub struct Solution;

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut k = 0;
        for (i, &p) in prices.iter().enumerate() {
            if i > 0 && p + 1 == prices[i - 1] {
                k += 1;
            } else {
                result += k * (k + 1) / 2;
                k = 1;
            }
        }
        result + k * (k + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            68,
            Solution::get_descent_periods(vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 4, 3, 10, 9, 8, 7])
        );
    }
}
