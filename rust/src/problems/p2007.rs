pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
        let n = changed.len();
        if !n.is_multiple_of(2) {
            return vec![];
        }
        changed.sort_unstable();
        let mut m = HashMap::new();
        for &e in &changed {
            *m.entry(e).or_insert(0) += 1;
        }
        let mut result = Vec::with_capacity(n / 2);

        let mut nums = m.keys().cloned().collect::<Vec<_>>();
        nums.sort_unstable();
        for e in nums {
            if let Some(c) = m.remove(&e) {
                if e == 0 {
                    if c % 2 != 0 {
                        return vec![];
                    } else {
                        result.resize(result.len() + c / 2, 0);
                    }
                } else if let Some(c2) = m.get_mut(&(2 * e)) {
                    if *c2 < c {
                        return vec![];
                    } else {
                        *c2 -= c;
                        for _ in 0..c {
                            result.push(e);
                        }
                        if *c2 == 0 {
                            m.remove(&(2 * e));
                        }
                    }
                } else {
                    return vec![];
                }
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
            vec![1, 3, 4],
            Solution::find_original_array(vec![1, 3, 4, 2, 6, 8])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![0; 0], Solution::find_original_array(vec![6, 3, 0, 1]));
    }
}
