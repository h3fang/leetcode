pub struct Solution;

use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let set = arr.iter().cloned().collect::<BTreeSet<_>>();
        let mut m = HashMap::new();
        for (i, n) in set.iter().enumerate() {
            m.insert(*n, i as i32 + 1);
        }
        arr.iter().map(|n| *m.get(n).unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![4, 1, 2, 3],
            Solution::array_rank_transform(vec![40, 10, 20, 30])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![1, 1, 1], Solution::array_rank_transform(vec![6, 6, 6]));
    }
}
