use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut c = HashMap::new();
        for n in candidates {
            *c.entry(n).or_insert(0) += 1;
        }
        let kv = c.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>();
        let mut max = 0;
        for i in 0..25 {
            let bit = 1 << i;
            let mut c = 0;
            for e in &kv {
                if bit & e.0 > 0 {
                    c += e.1;
                }
            }
            max = max.max(c);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            4,
            Solution::largest_combination(vec![16, 17, 71, 62, 12, 24, 14])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::largest_combination(vec![8, 8]));
    }
}
