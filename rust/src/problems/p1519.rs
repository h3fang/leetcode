pub struct Solution;

impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in &edges {
            g[e[0] as usize].push(e[1]);
            g[e[1] as usize].push(e[0]);
        }
        let mut result = vec![0; n];
        fn dfs(
            g: &[Vec<i32>],
            labels: &[u8],
            parent: i32,
            node: i32,
            result: &mut [i32],
        ) -> [i32; 26] {
            let mut f = [0; 26];
            let i = (labels[node as usize] - b'a') as usize;
            f[i] = 1;
            for &child in &g[node as usize] {
                if child == parent {
                    continue;
                }
                let f1 = dfs(g, labels, node, child, result);
                f.iter_mut().zip(&f1).for_each(|(a, b)| *a += b);
            }
            result[node as usize] = f[i];
            f
        }
        dfs(&g, labels.as_bytes(), -1, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 7;
        let edges = [[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let labels = "abaedcd".to_string();
        assert_eq!(
            vec![2, 1, 1, 1, 1, 1, 1],
            Solution::count_sub_trees(n, edges, labels)
        );
    }

    #[test]
    fn case2() {
        let n = 4;
        let edges = [[0, 1], [1, 2], [0, 3]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let labels = "bbbb".to_string();
        assert_eq!(
            vec![4, 2, 1, 1],
            Solution::count_sub_trees(n, edges, labels)
        );
    }

    #[test]
    fn case3() {
        let n = 5;
        let edges = [[0, 1], [0, 2], [1, 3], [0, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        let labels = "aabab".to_string();
        assert_eq!(
            vec![3, 2, 1, 1, 1],
            Solution::count_sub_trees(n, edges, labels)
        );
    }
}
