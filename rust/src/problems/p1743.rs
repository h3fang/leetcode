pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::with_capacity(adjacent_pairs.len());
        let mut m = HashMap::new();
        for p in adjacent_pairs {
            m.entry(p[0]).or_insert(vec![]).push(p[1]);
            m.entry(p[1]).or_insert(vec![]).push(p[0]);
        }
        let mut prev = m
            .iter()
            .find(|(_, v)| v.len() == 1)
            .map(|e| e.0)
            .cloned()
            .unwrap();
        result.push(prev);

        let mut curr = m.get(&prev).unwrap()[0];
        result.push(curr);

        loop {
            let arr = m.get(&curr).unwrap();
            if arr.len() == 1 {
                break;
            }
            let next = if arr[0] == prev { arr[1] } else { arr[0] };
            result.push(next);
            prev = curr;
            curr = next;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(adjacent_pairs: Vec<Vec<i32>>, arr: Vec<i32>) {
        let mut m = HashMap::new();
        for (i, x) in arr.into_iter().enumerate() {
            m.insert(x, i as i32);
        }
        assert!(adjacent_pairs.into_iter().all(|p| {
            let (i, j) = (m[&p[0]], m[&p[1]]);
            (i - j).abs() == 1
        }))
    }

    #[test]
    fn case1() {
        let adjacent_pairs = [[2, 1], [3, 4], [3, 2]]
            .iter()
            .map(|p| p.to_vec())
            .collect::<Vec<_>>();
        check(
            adjacent_pairs.clone(),
            Solution::restore_array(adjacent_pairs),
        );
    }

    #[test]
    fn case2() {
        let adjacent_pairs = [[4, -2], [1, 4], [-3, 1]]
            .iter()
            .map(|p| p.to_vec())
            .collect::<Vec<_>>();
        check(
            adjacent_pairs.clone(),
            Solution::restore_array(adjacent_pairs),
        );
    }

    #[test]
    fn case3() {
        let adjacent_pairs = [[100000, -100000]]
            .iter()
            .map(|p| p.to_vec())
            .collect::<Vec<_>>();
        check(
            adjacent_pairs.clone(),
            Solution::restore_array(adjacent_pairs),
        );
    }
}
