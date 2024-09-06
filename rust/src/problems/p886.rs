pub struct Solution;

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut g = vec![vec![]; n + 1];
        for d in dislikes {
            g[d[0] as usize].push(d[1]);
            g[d[1] as usize].push(d[0]);
        }
        let mut colors = vec![0; n + 1];
        fn dfs(g: &[Vec<i32>], colors: &mut [i32], i: usize, c: i32) -> bool {
            colors[i] = c;
            g[i].iter().all(|&y| {
                colors[y as usize] != c
                    && (colors[y as usize] != 0 || dfs(g, colors, y as usize, -c))
            })
        }
        (1..=n).all(|i| colors[i] != 0 || dfs(&g, &mut colors, i, 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 4;
        let dislikes = [[1, 2], [1, 3], [2, 4]]
            .iter()
            .map(|d| d.to_vec())
            .collect();
        assert!(Solution::possible_bipartition(n, dislikes));
    }

    #[test]
    fn case2() {
        let n = 3;
        let dislikes = [[1, 2], [1, 3], [2, 3]]
            .iter()
            .map(|d| d.to_vec())
            .collect();
        assert!(!Solution::possible_bipartition(n, dislikes));
    }

    #[test]
    fn case3() {
        let n = 5;
        let dislikes = [[1, 2], [2, 3], [3, 4], [4, 5], [1, 5]]
            .iter()
            .map(|d| d.to_vec())
            .collect();
        assert!(!Solution::possible_bipartition(n, dislikes));
    }
}
