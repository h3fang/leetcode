pub struct Solution;

impl Solution {
    pub fn minimum_removal(mut beans: Vec<i32>) -> i64 {
        let n = beans.len();
        beans.sort_unstable();
        let mut presum = vec![0; n];
        presum[0] = beans[0] as i64;
        for i in 1..presum.len() {
            presum[i] = presum[i - 1] + beans[i] as i64;
        }
        let mut result = i64::MAX;
        for (i, b) in beans.into_iter().enumerate() {
            let c = presum[n - 1] as i64 - b as i64 * (n - i) as i64;
            result = result.min(c);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::minimum_removal(vec![4, 1, 6, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::minimum_removal(vec![2, 10, 3, 2]));
    }
}
