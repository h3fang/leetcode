pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_maximum_elegance(mut items: Vec<Vec<i32>>, k: i32) -> i64 {
        let k = k as usize;
        items.sort_unstable_by_key(|e| -e[0]);
        let mut s = HashSet::new();
        let mut q = Vec::with_capacity(items.len());
        let mut profit = 0;
        for e in &items[..k] {
            profit += e[0] as i64;
            if s.contains(&e[1]) {
                q.push(e[0]);
            } else {
                s.insert(e[1]);
            }
        }
        let mut result = profit + (s.len() * s.len()) as i64;
        for a in &items[k..] {
            if !s.contains(&a[1]) && !q.is_empty() {
                let b = q.pop().unwrap();
                profit += (a[0] - b) as i64;
                s.insert(a[1]);
            }
            let score = profit + (s.len() * s.len()) as i64;
            result = result.max(score);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let items = [[3, 2], [5, 1], [10, 1]]
            .iter()
            .map(|i| i.to_vec())
            .collect();
        assert_eq!(17, Solution::find_maximum_elegance(items, 2));
    }

    #[test]
    fn case2() {
        let items = [[3, 1], [3, 1], [2, 2], [5, 3]]
            .iter()
            .map(|i| i.to_vec())
            .collect();
        assert_eq!(19, Solution::find_maximum_elegance(items, 3));
    }

    #[test]
    fn case3() {
        let items = [[1, 1], [2, 1], [3, 1]]
            .iter()
            .map(|i| i.to_vec())
            .collect();
        assert_eq!(7, Solution::find_maximum_elegance(items, 3));
    }
}
