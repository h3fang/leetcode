use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        for t in tasks {
            *count.entry(t).or_insert(0) += 1;
        }
        let mut result = 0;
        for &v in count.values() {
            if v == 1 {
                return -1;
            }
            if v % 3 == 0 {
                result += v / 3;
            } else if v % 3 == 2 {
                result += v / 3 + 1;
            } else {
                result += (v - 4) / 3 + 2;
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
            4,
            Solution::minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::minimum_rounds(vec![2, 3, 3]));
    }
}
