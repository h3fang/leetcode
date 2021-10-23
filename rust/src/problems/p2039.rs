use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let n = patience.len();
        let mut g = vec![vec![]; n];

        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }

        let mut dist = vec![i32::MAX; n];
        dist[0] = 0;
        let mut q = BinaryHeap::new();
        q.push((0, 0));

        while let Some((node, d)) = q.pop() {
            for neighbor in &g[node] {
                if -d + 1 < dist[*neighbor] {
                    dist[*neighbor] = -d + 1;
                    q.push((*neighbor, d - 1));
                }
            }
        }

        let mut result = 0;

        for (d, p) in dist.into_iter().zip(patience).skip(1) {
            let last = (2 * d - 1) / p;
            let last = last * p;
            result = result.max(last + 2 * d);
        }

        result + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = vec![vec![0, 1], vec![1, 2]];
        let patience = vec![0, 2, 1];
        assert_eq!(8, Solution::network_becomes_idle(edges, patience));
    }

    #[test]
    fn case2() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
        let patience = vec![0, 10, 10];
        assert_eq!(3, Solution::network_becomes_idle(edges, patience));
    }

    #[test]
    fn case3() {
        let edges = vec![
            vec![5, 7],
            vec![15, 18],
            vec![12, 6],
            vec![5, 1],
            vec![11, 17],
            vec![3, 9],
            vec![6, 11],
            vec![14, 7],
            vec![19, 13],
            vec![13, 3],
            vec![4, 12],
            vec![9, 15],
            vec![2, 10],
            vec![18, 4],
            vec![5, 14],
            vec![17, 5],
            vec![16, 2],
            vec![7, 1],
            vec![0, 16],
            vec![10, 19],
            vec![1, 8],
        ];
        let patience = vec![0, 2, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1];
        assert_eq!(67, Solution::network_becomes_idle(edges, patience));
    }

    #[test]
    fn case4() {
        let edges = vec![
            vec![0, 5],
            vec![3, 7],
            vec![2, 6],
            vec![10, 12],
            vec![11, 1],
            vec![9, 3],
            vec![0, 4],
            vec![5, 9],
            vec![7, 8],
            vec![4, 10],
            vec![8, 2],
            vec![1, 6],
            vec![12, 11],
        ];
        let patience = vec![0, 1, 7, 3, 6, 3, 6, 1, 1, 4, 3, 2, 1];
        assert_eq!(20, Solution::network_becomes_idle(edges, patience));
    }
}
