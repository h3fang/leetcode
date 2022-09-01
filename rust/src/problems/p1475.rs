pub struct Solution;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut s = Vec::with_capacity(prices.len());
        let mut result = vec![0; prices.len()];
        for (i, &p) in prices.iter().enumerate().rev() {
            while !s.is_empty() && *s.last().unwrap() > p {
                s.pop();
            }
            result[i] = p - s.last().cloned().unwrap_or(0);
            s.push(p);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![4, 2, 4, 2, 3],
            Solution::final_prices(vec![8, 4, 6, 2, 3])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![9, 0, 1, 6], Solution::final_prices(vec![10, 1, 1, 6]));
    }
}
