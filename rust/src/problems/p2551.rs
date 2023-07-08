pub struct Solution;

impl Solution {
    pub fn put_marbles(mut weights: Vec<i32>, k: i32) -> i64 {
        let n = weights.len();
        let k = k as usize;
        for i in 0..n - 1 {
            weights[i] += weights[i + 1];
        }
        weights.pop();
        weights.sort_unstable();
        weights
            .iter()
            .rev()
            .take(k - 1)
            .map(|&w| w as i64)
            .sum::<i64>()
            - weights.iter().take(k - 1).map(|&w| w as i64).sum::<i64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::put_marbles(vec![1, 3, 5, 1], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::put_marbles(vec![1, 3], 2));
    }
}
