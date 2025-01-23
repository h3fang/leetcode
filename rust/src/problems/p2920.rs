pub struct Solution;

fn dfs(
    i: usize,
    t: usize,
    p: i32,
    g: &[Vec<i32>],
    coins: &[i32],
    k: i32,
    f: &mut [[i32; 14]],
) -> i32 {
    if t >= 14 {
        return 0;
    }
    if f[i][t] >= 0 {
        return f[i][t];
    }
    let mut a = (coins[i] >> t) - k;
    let mut b = coins[i] >> (t + 1);
    for &j in &g[i] {
        if j == p {
            continue;
        }
        a += dfs(j as usize, t, i as i32, g, coins, k, f);
        b += dfs(j as usize, t + 1, i as i32, g, coins, k, f);
    }
    let r = a.max(b);
    f[i][t] = r;
    r
}

impl Solution {
    pub fn maximum_points(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
        let n = coins.len();
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        let mut f = vec![[-1; 14]; n];
        dfs(0, 0, -1, &g, &coins, k, &mut f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1], [1, 2], [2, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let coins = vec![10, 10, 3, 3];
        assert_eq!(11, Solution::maximum_points(edges, coins, 5));
    }

    #[test]
    fn case2() {
        let edges = [[0, 1], [0, 2]].iter().map(|e| e.to_vec()).collect();
        let coins = vec![8, 4, 4];
        assert_eq!(16, Solution::maximum_points(edges, coins, 0));
    }
}
