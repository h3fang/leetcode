pub struct Solution;

fn dfs(g: &[Vec<i32>], cost: &[i32], x: i32, p: i32) -> (i32, i64) {
    let neighbors = &g[x as usize];
    let (mut changes, mut max, mut count) = (0, 0, 0);
    for &y in neighbors {
        if y == p {
            continue;
        }
        let (c1, m1) = dfs(g, cost, y, x);
        match m1.cmp(&max) {
            std::cmp::Ordering::Less => {}
            std::cmp::Ordering::Equal => count += 1,
            std::cmp::Ordering::Greater => {
                max = m1;
                count = 1;
            }
        }
        changes += c1;
    }
    (
        changes + neighbors.len() as i32 - count - 1,
        max + cost[x as usize] as i64,
    )
}

impl Solution {
    pub fn min_increase(n: i32, edges: Vec<Vec<i32>>, cost: Vec<i32>) -> i32 {
        let mut g = vec![vec![]; n as usize];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        g[0].push(-1);
        dfs(&g, &cost, 0, -1).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1], [0, 2]].iter().map(|e| e.to_vec()).collect();
        let cost = vec![2, 1, 3];
        assert_eq!(1, Solution::min_increase(3, edges, cost));
    }

    #[test]
    fn case2() {
        let edges = [[0, 1], [1, 2]].iter().map(|e| e.to_vec()).collect();
        let cost = vec![5, 1, 4];
        assert_eq!(0, Solution::min_increase(3, edges, cost));
    }

    #[test]
    fn case3() {
        let edges = [[0, 4], [0, 1], [1, 2], [1, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let cost = vec![3, 4, 1, 1, 7];
        assert_eq!(1, Solution::min_increase(5, edges, cost));
    }

    #[test]
    fn case4() {
        let edges = [[0, 1], [1, 2], [1, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let cost = vec![13, 7, 9, 4];
        assert_eq!(1, Solution::min_increase(4, edges, cost));
    }

    #[test]
    fn case5() {
        let edges = [[0, 1], [0, 2], [0, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let cost = vec![10, 12, 7, 13];
        assert_eq!(2, Solution::min_increase(4, edges, cost));
    }
}
