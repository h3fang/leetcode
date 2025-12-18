pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let (mut sum, mut max, mut curr) = (0, 0, 0);
        for (i, (p, s)) in prices.iter().zip(&strategy).enumerate() {
            sum += (p * s) as i64;
            curr += (p * (1 - s)) as i64;
            if i + 1 < k {
                if 2 * (i + 1) >= k {
                    curr -= prices[i + 1 - k / 2] as i64;
                }
                continue;
            }
            max = max.max(curr);
            curr += (prices[i + 1 - k] * strategy[i + 1 - k]) as i64;
            curr -= prices[i + 1 - k / 2] as i64;
        }
        sum + max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(10, Solution::max_profit(vec![4, 2, 8], vec![-1, 0, 1], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(9, Solution::max_profit(vec![5, 4, 3], vec![1, 1, 0], 2));
    }
}
