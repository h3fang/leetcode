pub struct Solution;

const N: usize = 100001;
static PRIMES: [bool; N] = {
    let mut primes = [true; N];
    primes[1] = false;
    let mut i = 2;
    while i * i < N {
        if primes[i] {
            let mut j = i * i;
            while j < N {
                primes[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    primes
};

impl Solution {
    pub fn count_paths(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut g = vec![vec![]; n as usize + 1];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        let mut sizes = vec![0; n as usize + 1];
        fn dfs(g: &[Vec<i32>], x: i32, p: i32, nodes: &mut Vec<i32>) {
            nodes.push(x);
            for &y in &g[x as usize] {
                if y != p && !PRIMES[y as usize] {
                    dfs(g, y, x, nodes);
                }
            }
        }
        let mut result = 0;
        for x in 2..n + 1 {
            if !PRIMES[x as usize] {
                continue;
            }
            let mut sum = 0;
            for &y in &g[x as usize] {
                if PRIMES[y as usize] {
                    continue;
                }
                if sizes[y as usize] == 0 {
                    let mut nodes = vec![];
                    dfs(&g, y, -1, &mut nodes);
                    let m = nodes.len();
                    for e in nodes {
                        sizes[e as usize] = m;
                    }
                }
                result += sum * sizes[y as usize];
                sum += sizes[y as usize];
            }
            result += sum;
        }
        result as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[1, 2], [1, 3], [2, 4], [2, 5]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(4, Solution::count_paths(5, edges));
    }

    #[test]
    fn case2() {
        let edges = [[1, 2], [1, 3], [2, 4], [3, 5], [3, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(6, Solution::count_paths(6, edges));
    }
}
