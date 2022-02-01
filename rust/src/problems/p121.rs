pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut min = i32::MAX;
        for p in prices {
            if p < min {
                min = p;
            }
            result = result.max(p - min);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }
}
