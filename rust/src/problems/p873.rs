pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let map: HashMap<i32, usize> = arr.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        let mut dp = vec![vec![0; n]; n];
        let mut result = 0;
        for i in 0..n {
            for j in (0..i).rev() {
                let nk = arr[i] - arr[j];
                if let Some(&k) = map.get(&nk) {
                    if k >= j {
                        break;
                    }
                    dp[j][i] = (dp[k][j] + 1).max(3);
                    result = result.max(dp[j][i]);
                }
            }
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
            5,
            Solution::len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            3,
            Solution::len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18])
        );
    }
}
