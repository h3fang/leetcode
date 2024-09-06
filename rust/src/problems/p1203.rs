pub struct Solution;

use std::collections::{HashMap, VecDeque};

fn topological_sort(g: &[Vec<i32>], deg: &mut [i32]) -> Vec<i32> {
    let mut q = VecDeque::new();
    deg.iter().enumerate().for_each(|(i, &d)| {
        if d == 0 {
            q.push_back(i as i32);
        }
    });
    let mut result = Vec::with_capacity(deg.len());
    while let Some(x) = q.pop_front() {
        result.push(x);
        for &y in &g[x as usize] {
            deg[y as usize] -= 1;
            if deg[y as usize] == 0 {
                q.push_back(y);
            }
        }
    }
    result
}

impl Solution {
    pub fn sort_items(
        n: i32,
        mut m: i32,
        mut group: Vec<i32>,
        before_items: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        for e in group.iter_mut() {
            if *e == -1 {
                *e = m;
                m += 1;
            }
        }

        let (n, m) = (n as usize, m as usize);
        let mut adj = vec![vec![]; m];
        let mut deg = vec![0; m];

        for (i, &x) in group.iter().enumerate() {
            for &j in &before_items[i] {
                let y = group[j as usize];
                if x != y {
                    adj[y as usize].push(x);
                    deg[x as usize] += 1;
                }
            }
        }

        let groups_order = topological_sort(&adj, &mut deg);
        if groups_order.len() != m {
            return vec![];
        }

        let mut adj = vec![vec![]; n];
        let mut deg = vec![0; n];

        for (i, items) in before_items.iter().enumerate() {
            for &item in items {
                adj[item as usize].push(i as i32);
                deg[i] += 1;
            }
        }

        let items_order = topological_sort(&adj, &mut deg);
        if items_order.len() != n {
            return vec![];
        }

        let mut m = HashMap::new();
        for item in items_order {
            m.entry(group[item as usize]).or_insert(vec![]).push(item);
        }
        let mut result = Vec::with_capacity(n);
        for g in groups_order {
            result.extend(m.get(&g).unwrap_or(&vec![]));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn check(n: i32, group: &[i32], before_items: &[Vec<i32>], result: &[i32]) {
        assert!(result.len() == n as usize);

        let mut group_indices: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &item) in result.iter().enumerate() {
            group_indices
                .entry(group[item as usize])
                .or_default()
                .push(i);
        }

        assert!(group_indices
            .values()
            .all(|v| v.windows(2).all(|w| w[0] + 1 == w[1])));

        let mut seen = HashSet::with_capacity(n as usize);
        for &item in result {
            assert!(before_items[item as usize].iter().all(|x| seen.contains(x)));
            seen.insert(item);
        }
    }

    #[test]
    fn case1() {
        let (n, m) = (8, 2);
        let group = vec![-1, -1, 1, 0, 0, 1, 0, -1];
        let before_items = vec![
            vec![],
            vec![6],
            vec![5],
            vec![6],
            vec![3, 6],
            vec![],
            vec![],
            vec![],
        ];
        let result = Solution::sort_items(n, m, group.clone(), before_items.clone());
        check(n, &group, &before_items, &result);
    }

    #[test]
    fn case2() {
        let (n, m) = (8, 2);
        let group = vec![-1, -1, 1, 0, 0, 1, 0, -1];
        let before_items = vec![
            vec![],
            vec![6],
            vec![5],
            vec![6],
            vec![3],
            vec![],
            vec![4],
            vec![],
        ];
        let result = Solution::sort_items(n, m, group.clone(), before_items.clone());
        assert!(result.is_empty());
    }
}
