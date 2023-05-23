pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let n = values.len();
        let mut id = (0..n).collect::<Vec<_>>();
        id.sort_unstable_by_key(|&i| -values[i]);
        let mut result = 0;
        let mut chosen = 0;
        let mut count = HashMap::new();
        for i in id {
            if chosen >= num_wanted {
                break;
            }
            let label = labels[i];
            if *count.get(&label).unwrap_or(&0) == use_limit {
                continue;
            }
            chosen += 1;
            result += values[i];
            *count.entry(label).or_default() += 1;
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
            9,
            Solution::largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 1, 2, 2, 3], 3, 1)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            12,
            Solution::largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 3, 3, 3, 2], 3, 2)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            16,
            Solution::largest_vals_from_labels(vec![9, 8, 8, 7, 6], vec![0, 0, 0, 1, 1], 3, 1)
        );
    }
}
