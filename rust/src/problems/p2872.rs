pub struct Solution;

impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        fn dfs(g: &[Vec<i32>], i: i32, parent: i32, values: &[i32], k: i64) -> (i32, i64) {
            let (mut result, mut sum) = (0, values[i as usize] as i64);
            for &j in &g[i as usize] {
                if j == parent {
                    continue;
                }
                let (c, s) = dfs(g, j, i, values, k);
                result += c;
                sum += s;
            }
            if sum % k == 0 {
                (result + 1, 0)
            } else {
                (result, sum)
            }
        }

        let mut g = vec![vec![]; n as usize];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        dfs(&g, 0, -1, &values, k as i64).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 2], [1, 2], [1, 3], [2, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let values = vec![1, 8, 1, 4, 4];
        assert_eq!(2, Solution::max_k_divisible_components(5, edges, values, 6));
    }

    #[test]
    fn case2() {
        let edges = [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let values = vec![3, 0, 6, 1, 5, 2, 1];
        assert_eq!(3, Solution::max_k_divisible_components(7, edges, values, 3));
    }
}
