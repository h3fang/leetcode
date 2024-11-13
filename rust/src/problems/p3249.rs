pub struct Solution;

impl Solution {
    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        fn dfs(root: i32, parent: i32, g: &[Vec<i32>], result: &mut i32) -> u32 {
            let (mut size, mut child_size, mut good) = (1, 0, true);
            for &c in &g[root as usize] {
                if c == parent {
                    continue;
                }
                let s = dfs(c, root, g, result);
                if child_size == 0 {
                    child_size = s;
                } else if child_size != s {
                    good = false;
                }
                size += s;
            }
            if good {
                *result += 1;
            }
            size
        }
        let mut g = vec![vec![]; edges.len() + 1];
        for e in edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        let mut result = 0;
        dfs(0, -1, &g, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(7, Solution::count_good_nodes(edges));
    }

    #[test]
    fn case2() {
        let edges = [
            [0, 1],
            [1, 2],
            [2, 3],
            [3, 4],
            [0, 5],
            [1, 6],
            [2, 7],
            [3, 8],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        assert_eq!(6, Solution::count_good_nodes(edges));
    }

    #[test]
    fn case3() {
        let edges = [
            [0, 1],
            [1, 2],
            [1, 3],
            [1, 4],
            [0, 5],
            [5, 6],
            [6, 7],
            [7, 8],
            [0, 9],
            [9, 10],
            [9, 12],
            [10, 11],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        assert_eq!(12, Solution::count_good_nodes(edges));
    }
}
