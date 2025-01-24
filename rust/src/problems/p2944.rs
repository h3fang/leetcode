pub struct Solution;

impl Solution {
    pub fn minimum_coins(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut f = vec![i32::MAX; n];
        f[n - 1] = prices[n - 1];
        for i in (0..n - 1).rev() {
            f[i] = prices[i]
                + if i + i + 2 >= n {
                    0
                } else {
                    *f[i + 1..i + i + 3].iter().min().unwrap()
                };
        }
        f[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::minimum_coins(vec![3, 1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::minimum_coins(vec![1, 10, 1, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            39,
            Solution::minimum_coins(vec![26, 18, 6, 12, 49, 7, 45, 45])
        );
    }
}
