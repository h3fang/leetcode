pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut m = HashMap::new();
        m.insert(0, 1);
        let mut sum = 0;
        let mut result = 0;
        for n in nums {
            sum += n;
            let modulus = ((sum % k) + k) % k;
            if let Some(c) = m.get(&modulus) {
                result += c;
            }
            *m.entry(modulus).or_default() += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(7, Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::subarrays_div_by_k(vec![5], 9));
    }
}
