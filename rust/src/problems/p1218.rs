use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut seqs = HashMap::new();
        let mut result = 1;
        for n in arr {
            let k = seqs.get(&(n - difference)).map(|k| k + 1).unwrap_or(1);
            result = result.max(k);
            seqs.insert(n, k);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let arr = vec![1, 2, 3, 4];
        let difference = 1;
        assert_eq!(4, Solution::longest_subsequence(arr, difference));
    }

    #[test]
    fn case2() {
        let arr = vec![1, 3, 5, 7];
        let difference = 1;
        assert_eq!(1, Solution::longest_subsequence(arr, difference));
    }

    #[test]
    fn case3() {
        let arr = vec![1, 5, 7, 8, 5, 3, 4, 2, 1];
        let difference = -2;
        assert_eq!(4, Solution::longest_subsequence(arr, difference));
    }
}
