use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    pub fn maximal_path_quality(mut values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        #[allow(clippy::too_many_arguments)]
        fn dfs(
            g: &HashMap<i32, Vec<(i32, i32)>>,
            node: i32,
            time: i32,
            max_time: i32,
            value: i32,
            min_times: &[i32],
            values: &mut Vec<i32>,
            result: &mut i32,
        ) {
            let v = values[node as usize];
            if node == 0 {
                *result = (*result).max(value + v);
            }
            if let Some(neighbors) = g.get(&node) {
                values[node as usize] = 0;
                for (next, t) in neighbors {
                    if time + t + min_times[*next as usize] <= max_time {
                        dfs(
                            g,
                            *next,
                            time + t,
                            max_time,
                            value + v,
                            min_times,
                            values,
                            result,
                        );
                    }
                }
                values[node as usize] = v;
            }
        }

        fn dijkstra(g: &HashMap<i32, Vec<(i32, i32)>>, min_times: &mut [i32]) {
            let mut q = BinaryHeap::new();
            let mut visited = vec![false; min_times.len()];
            q.push((0, 0));
            min_times[0] = 0;

            while let Some((time, node)) = q.pop() {
                let time = -time;
                visited[node as usize] = true;
                if let Some(neighbors) = g.get(&node) {
                    for &(next, weight) in neighbors {
                        if !visited[next as usize] && time + weight < min_times[next as usize] {
                            min_times[next as usize] = time + weight;
                            q.push((-time - weight, next));
                        }
                    }
                }
            }
        }

        let mut g: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        for e in &edges {
            g.entry(e[0]).or_default().push((e[1], e[2]));
            g.entry(e[1]).or_default().push((e[0], e[2]));
        }

        let mut min_times = vec![i32::MAX; values.len()];
        dijkstra(&g, &mut min_times);
        let mut result = 0;
        dfs(&g, 0, 0, max_time, 0, &min_times, &mut values, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let values = vec![0, 32, 10, 43];
        let edges = vec![vec![0, 1, 10], vec![1, 2, 15], vec![0, 3, 10]];
        let max_time = 49;
        assert_eq!(75, Solution::maximal_path_quality(values, edges, max_time));
    }

    #[test]
    fn case2() {
        let values = vec![5, 10, 15, 20];
        let edges = vec![vec![0, 1, 10], vec![1, 2, 10], vec![0, 3, 10]];
        let max_time = 30;
        assert_eq!(25, Solution::maximal_path_quality(values, edges, max_time));
    }

    #[test]
    fn case3() {
        let values = vec![1, 2, 3, 4];
        let edges = vec![
            vec![0, 1, 10],
            vec![1, 2, 11],
            vec![2, 3, 12],
            vec![1, 3, 13],
        ];
        let max_time = 50;
        assert_eq!(7, Solution::maximal_path_quality(values, edges, max_time));
    }

    #[test]
    fn case4() {
        let values = vec![0, 1, 2];
        let edges = vec![vec![1, 2, 10]];
        let max_time = 10;
        assert_eq!(0, Solution::maximal_path_quality(values, edges, max_time));
    }
}
