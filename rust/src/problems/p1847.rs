pub struct Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut s: BTreeSet<i32> = BTreeSet::new();
        let mut result = vec![-1; queries.len()];
        let mut events: Vec<(i32, i32, i32)> = rooms
            .into_iter()
            .map(|r| (r[1], i32::MAX, r[0]))
            .chain(
                queries
                    .into_iter()
                    .enumerate()
                    .map(|(i, q)| (q[1], i as i32, q[0])),
            )
            .collect();
        events.sort_unstable();
        for (_, i, id) in events.into_iter().rev() {
            if i == i32::MAX {
                s.insert(id);
            } else {
                let (mut min, mut r) = (i32::MAX, i32::MAX);
                if let Some(&j) = s.range(id..).next() {
                    min = (j - id).abs();
                    r = j;
                }
                if let Some(&j) = s.range(..id).next_back() {
                    if (j - id).abs() <= min {
                        r = r.min(j);
                    }
                }
                if r != i32::MAX {
                    result[i as usize] = r;
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
        let rooms = [[2, 2], [1, 2], [3, 2]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        let queries = [[3, 1], [3, 3], [5, 2]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(vec![3, -1, 3], Solution::closest_room(rooms, queries));
    }

    #[test]
    fn case2() {
        let rooms = [[1, 4], [2, 3], [3, 5], [4, 1], [5, 2]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        let queries = [[2, 3], [2, 4], [2, 5]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(vec![2, 1, 3], Solution::closest_room(rooms, queries));
    }
}
