pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn minimum_coins(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut q = VecDeque::with_capacity(n);
        q.push_back((n + 1, 0));
        for i in (1..=n).rev() {
            while q.back().unwrap().0 > i + i + 1 {
                q.pop_back();
            }
            let p = prices[i - 1] + q.back().unwrap().1;
            while p < q.front().unwrap().1 {
                q.pop_front();
            }
            q.push_front((i, p));
        }
        q.front().unwrap().1
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
