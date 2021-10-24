pub struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(
            graph: &[Vec<i32>],
            n: i32,
            target: i32,
            curr: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if n == target {
                result.push(curr.clone());
            }

            for next in &graph[n as usize] {
                curr.push(*next);
                dfs(graph, *next, target, curr, result);
                curr.pop();
            }
        }

        let mut curr = vec![0];
        let mut result = Vec::new();
        dfs(&graph, 0, graph.len() as i32 - 1, &mut curr, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let expected = vec![vec![0, 1, 3], vec![0, 2, 3]];
        assert_eq!(expected, Solution::all_paths_source_target(graph));
    }

    #[test]
    fn case2() {
        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        let expected = vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ];
        assert_eq!(expected, Solution::all_paths_source_target(graph));
    }

    #[test]
    fn case3() {
        let graph = vec![vec![1], vec![]];
        let expected = vec![vec![0, 1]];
        assert_eq!(expected, Solution::all_paths_source_target(graph));
    }

    #[test]
    fn case4() {
        let graph = vec![vec![1, 2, 3], vec![2], vec![3], vec![]];
        let expected = vec![vec![0, 1, 2, 3], vec![0, 2, 3], vec![0, 3]];
        assert_eq!(expected, Solution::all_paths_source_target(graph));
    }

    #[test]
    fn case5() {
        let graph = vec![vec![1, 3], vec![2], vec![3], vec![]];
        let expected = vec![vec![0, 1, 2, 3], vec![0, 3]];
        assert_eq!(expected, Solution::all_paths_source_target(graph));
    }
}
