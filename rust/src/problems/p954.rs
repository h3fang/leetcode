use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        let mut m = BTreeMap::new();
        for n in arr {
            *m.entry(n).or_insert(0) += 1;
        }
        while !m.is_empty() {
            let (&k, &v) = m.iter().next().unwrap();
            m.remove(&k);
            if k == 0 {
                if v % 2 != 0 {
                    return false;
                }
                continue;
            }
            if k < 0 && k % 2 != 0 {
                return false;
            }
            let next = if k < 0 { k / 2 } else { k * 2 };
            match m.entry(next) {
                Entry::Vacant(_) => return false,
                Entry::Occupied(mut e) => {
                    let e = e.get_mut();
                    if *e < v {
                        return false;
                    } else {
                        *e -= v;
                        if *e == 0 {
                            m.remove(&next);
                        }
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::can_reorder_doubled(vec![3, 1, 3, 6]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::can_reorder_doubled(vec![2, 1, 2, 6]));
    }

    #[test]
    fn case3() {
        assert!(Solution::can_reorder_doubled(vec![4, -2, 2, -4]));
    }
}
