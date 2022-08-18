use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut count = HashMap::new();
        for e in arr {
            *count.entry(e).or_insert(0) += 1;
        }
        let mut c = count.into_values().collect::<Vec<_>>();
        c.sort_unstable();
        let mut sum = 0;
        let half = (n + 1) / 2;
        for (i, f) in c.into_iter().rev().enumerate() {
            sum += f;
            if sum >= half {
                return i as i32 + 1;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::min_set_size(vec![7, 7, 7, 7, 7, 7]));
    }
}
