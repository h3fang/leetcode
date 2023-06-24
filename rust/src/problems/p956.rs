pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        let mut m = HashMap::new();
        m.insert(0, 0);
        for r in rods {
            let t = m.clone();
            for (k, v) in t {
                m.insert(k + r, m.get(&(k + r)).cloned().unwrap_or(0).max(v + r));
                m.insert(k - r, m.get(&(k - r)).cloned().unwrap_or(0).max(v));
            }
        }
        m[&0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::tallest_billboard(vec![1, 2, 3, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::tallest_billboard(vec![1, 2, 3, 4, 5, 6]));
    }
    #[test]
    fn case3() {
        assert_eq!(0, Solution::tallest_billboard(vec![1, 2]));
    }
}
