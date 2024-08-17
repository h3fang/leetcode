pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
        let mut m = HashMap::new();
        let w = word.as_bytes();
        let max = w.len() as i32 / k;
        let mut result = max;
        for s in w.chunks(k as usize) {
            let e = m.entry(s).or_insert(0);
            *e += 1;
            result = result.min(max - *e);
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
            1,
            Solution::minimum_operations_to_make_k_periodic("leetcodeleet".to_string(), 4)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            3,
            Solution::minimum_operations_to_make_k_periodic("leetcoleet".to_string(), 2)
        );
    }
}
