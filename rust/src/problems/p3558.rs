pub struct Solution;

fn dfs(g: &[Vec<i32>], x: i32, p: i32) -> i32 {
    let mut d = 0;
    for &y in &g[x as usize] {
        if y != p {
            d = d.max(dfs(g, y, x) + 1);
        }
    }
    d
}

const MOD: i64 = 10_0000_0007;

fn qpow(mut x: i64, mut n: i32) -> i64 {
    let mut ans = 1;
    while n > 0 {
        if n & 1 == 1 {
            ans = (ans * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    ans
}

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n + 1];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        let d = dfs(&g, 1, 0);
        qpow(2, d - 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[1, 2]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(1, Solution::assign_edge_weights(edges));
    }

    #[test]
    fn case2() {
        let edges = [[1, 2], [1, 3], [3, 4], [3, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(2, Solution::assign_edge_weights(edges));
    }
}
