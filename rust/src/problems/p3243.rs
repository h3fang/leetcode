pub struct Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prev = vec![vec![]; n as usize];
        for (i, p) in prev.iter_mut().enumerate().skip(1) {
            p.push(i as i32 - 1);
        }
        let mut f: Vec<i32> = (0..n).collect();
        queries
            .into_iter()
            .map(|q| {
                prev[q[1] as usize].push(q[0]);
                for v in q[1]..n {
                    for &u in &prev[v as usize] {
                        f[v as usize] = f[v as usize].min(f[u as usize] + 1);
                    }
                }
                f[n as usize - 1]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [[2, 4], [0, 2], [0, 4]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        assert_eq!(
            vec![3, 2, 1],
            Solution::shortest_distance_after_queries(5, queries)
        );
    }

    #[test]
    fn case2() {
        let queries = [[0, 3], [0, 2]].iter().map(|q| q.to_vec()).collect();
        assert_eq!(
            vec![1, 1],
            Solution::shortest_distance_after_queries(4, queries)
        );
    }
}
