pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut f: HashMap<i32, i32> = HashMap::new();
        for x in arr {
            *f.entry(x).or_default() += 1;
        }
        let mut f = f.into_values().collect::<Vec<_>>();
        f.sort_unstable();
        let n = f.len() as i32;
        let mut s = 0;
        for (i, x) in f.into_iter().enumerate() {
            if s + x > k {
                return n - i as i32;
            }
            s += x;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::find_least_num_of_unique_ints(vec![5, 5, 4], 1));
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3)
        );
    }
}
