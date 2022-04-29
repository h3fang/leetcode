pub struct Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut colors = vec![0; graph.len()];
        for i in 0..n {
            if colors[i] != 0 {
                continue;
            }
            colors[i] = 1;
            let mut q = vec![i];
            while let Some(node) = q.pop() {
                let c = colors[node];
                for &next in &graph[node as usize] {
                    let next = next as usize;
                    if colors[next] == c {
                        return false;
                    } else if colors[next] == 0 {
                        colors[next] = if c == 1 { 2 } else { 1 };
                        q.push(next);
                    }
                }
            }
        }

        colors.iter().all(|&c| c > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
        assert_eq!(false, Solution::is_bipartite(graph));
    }

    #[test]
    fn case2() {
        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert_eq!(true, Solution::is_bipartite(graph));
    }
}
