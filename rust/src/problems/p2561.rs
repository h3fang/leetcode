pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let n = basket1.len();
        let mut f = HashMap::with_capacity(n);
        let mut min = i32::MAX;
        for x in basket1 {
            *f.entry(x).or_insert(0i32) += 1;
            min = min.min(x);
        }
        for x in basket2 {
            *f.entry(x).or_insert(0) -= 1;
            min = min.min(x);
        }
        let mut swaps = Vec::with_capacity(n);
        for (x, f) in f {
            if f % 2 != 0 {
                return -1;
            }
            for _ in 0..f.abs() / 2 {
                swaps.push(x);
            }
        }
        swaps.sort_unstable();
        let n = swaps.len();
        let mut ans = 0;
        for x in swaps.into_iter().take(n / 2) {
            ans += x.min(2 * min) as i64;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_cost(vec![4, 2, 2, 2], vec![1, 4, 1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::min_cost(vec![2, 3, 4, 1], vec![3, 2, 5, 1]));
    }
}
