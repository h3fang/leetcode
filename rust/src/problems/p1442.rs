pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        for j in 1..arr.len() {
            let mut m = HashMap::new();
            let mut s = 0;
            for &x in arr[..j].iter().rev() {
                s ^= x;
                *m.entry(s).or_insert(0) += 1;
            }
            s = 0;
            for &x in &arr[j..] {
                s ^= x;
                result += m.get(&s).unwrap_or(&0);
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
        assert_eq!(4, Solution::count_triplets(vec![2, 3, 1, 6, 7]));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::count_triplets(vec![1, 1, 1, 1, 1]));
    }
}
