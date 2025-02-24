pub struct Solution;

fn dfs_bob(g: &[Vec<i32>], i: i32, parent: i32, d: i32, dist: &mut [i32]) -> bool {
    if i == 0 {
        dist[i as usize] = d;
        return true;
    }
    for &j in &g[i as usize] {
        if j != parent && dfs_bob(g, j, i, d + 1, dist) {
            dist[i as usize] = d;
            return true;
        }
    }
    false
}

#[allow(clippy::too_many_arguments)]
fn dfs_alice(
    g: &[Vec<i32>],
    i: i32,
    parent: i32,
    d: i32,
    mut profit: i32,
    dist: &[i32],
    amount: &[i32],
    result: &mut i32,
) {
    profit += match dist[i as usize].cmp(&(d)) {
        std::cmp::Ordering::Less => 0,
        std::cmp::Ordering::Equal => amount[i as usize] / 2,
        std::cmp::Ordering::Greater => amount[i as usize],
    };
    if i != 0 && g[i as usize].len() == 1 {
        *result = profit.max(*result);
        return;
    }
    for &j in &g[i as usize] {
        if j != parent {
            dfs_alice(g, j, i, d + 1, profit, dist, amount, result);
        }
    }
}

impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }

        let mut dist = vec![n as i32; n];
        dfs_bob(&g, bob, -1, 0, &mut dist);

        let mut result = i32::MIN;
        dfs_alice(&g, 0, -1, 0, 0, &dist, &amount, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1], [1, 2], [1, 3], [3, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(
            6,
            Solution::most_profitable_path(edges, 3, vec![-2, 4, 2, -4, 6])
        );
    }

    #[test]
    fn case2() {
        let edges = [[0, 1]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(
            -7280,
            Solution::most_profitable_path(edges, 1, vec![-7280, 2350])
        );
    }

    #[test]
    fn case3() {
        let edges = [[0, 1], [1, 2], [2, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(
            -11662,
            Solution::most_profitable_path(edges, 3, vec![-5644, -6018, 1188, -8502])
        );
    }
}
