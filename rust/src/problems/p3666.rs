pub struct Solution;

use std::collections::{BTreeSet, VecDeque};

impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let (n, s) = (s.len() as i32, s.as_bytes());

        let mut sets = [BTreeSet::new(), BTreeSet::new()];
        for i in 0..=n {
            sets[i as usize % 2].insert(i);
        }

        let m = s.iter().filter(|&&b| b == b'0').count() as i32;

        let mut q = VecDeque::with_capacity(n as usize + 1);
        q.push_back(m);
        sets[m as usize % 2].remove(&m);

        let mut ans = 0;

        while !q.is_empty() {
            let len = q.len();
            for _ in 0..len {
                let x = q.pop_front().unwrap();
                if x == 0 {
                    return ans;
                }
                let c1 = (k - n + x).max(0);
                let c2 = x.min(k);

                let l = x + k - 2 * c2;
                let r = x + k - 2 * c1;

                let set = &mut sets[l as usize % 2];
                let next: Vec<i32> = set.range(l..=r).cloned().collect();

                for y in next {
                    q.push_back(y);
                    set.remove(&y);
                }
            }
            ans += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::min_operations("110".to_string(), 1));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_operations("0101".to_string(), 3));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::min_operations("101".to_string(), 2));
    }
}
