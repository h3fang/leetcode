pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();
        for n in nums {
            *m.entry(n).or_insert(0) += 1;
        }
        m.iter()
            .filter_map(|(k, v)| if *v == 1 { Some(*k) } else { None })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::sum_of_unique(vec![1, 2, 3, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::sum_of_unique(vec![1, 1, 1, 1, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(15, Solution::sum_of_unique(vec![1, 2, 3, 4, 5]));
    }
}
