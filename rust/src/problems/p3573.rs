pub struct Solution;

impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut f = vec![[i64::MIN / 2; 3]; k + 2];
        f.iter_mut().skip(1).for_each(|e| e[0] = 0);
        for p in prices {
            let p = p as i64;
            for j in (1..k + 2).rev() {
                f[j][0] = f[j][0].max(f[j][1] + p).max(f[j][2] - p);
                f[j][1] = f[j][1].max(f[j - 1][0] - p);
                f[j][2] = f[j][2].max(f[j - 1][0] + p);
            }
        }
        f[k + 1][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(14, Solution::maximum_profit(vec![1, 7, 9, 8, 2], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(
            36,
            Solution::maximum_profit(vec![12, 16, 19, 19, 8, 1, 19, 13, 9], 3)
        );
    }
}
