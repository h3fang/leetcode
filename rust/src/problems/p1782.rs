pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let mut deg = vec![0; n];
        let mut cnt = HashMap::new();
        for e in &edges {
            let a = e[0] as usize - 1;
            let b = e[1] as usize - 1;
            deg[a] += 1;
            deg[b] += 1;
            *cnt.entry((a.min(b), a.max(b))).or_insert(0) += 1;
        }
        let mut d = deg.clone();
        d.sort_unstable();
        queries
            .into_iter()
            .map(|q| {
                let mut total = 0;
                let mut j = n - 1;
                for i in 0..n {
                    while j > i && d[i] + d[j] > q {
                        j -= 1;
                    }
                    total += n - 1 - i.max(j);
                }
                for (&(a, b), &c) in &cnt {
                    if deg[a] + deg[b] > q && deg[a] + deg[b] - c <= q {
                        total -= 1;
                    }
                }
                total as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[1, 2], [2, 4], [1, 3], [2, 3], [2, 1]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let queries = vec![2, 3];
        assert_eq!(vec![6, 5], Solution::count_pairs(4, edges, queries));
    }

    #[test]
    fn case2() {
        let edges = [
            [1, 5],
            [1, 5],
            [3, 4],
            [2, 5],
            [1, 3],
            [5, 1],
            [2, 3],
            [2, 5],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        let queries = vec![1, 2, 3, 4, 5];
        assert_eq!(
            vec![10, 10, 9, 8, 6],
            Solution::count_pairs(5, edges, queries)
        );
    }
}
