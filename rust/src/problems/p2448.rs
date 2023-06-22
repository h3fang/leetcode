pub struct Solution;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let mut sum_c = cost.iter().map(|&e| e as i64).sum::<i64>();
        let mut a = nums
            .iter()
            .zip(&cost)
            .map(|(&a, &b)| (a as i64, b as i64))
            .collect::<Vec<_>>();
        a.sort_unstable();
        let mut total = a.iter().map(|(x, c)| (x - a[0].0) * c).sum::<i64>();
        let mut result = total;
        for w in a.windows(2) {
            sum_c -= w[0].1 * 2;
            total -= sum_c * (w[1].0 - w[0].0);
            result = result.min(total);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::min_cost(vec![1, 3, 5, 2], vec![2, 3, 1, 14]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::min_cost(vec![2, 2, 2, 2, 2], vec![4, 2, 8, 1, 3])
        );
    }
}
