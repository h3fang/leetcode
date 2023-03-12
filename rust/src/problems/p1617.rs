pub struct Solution;

impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut g = vec![vec![]; n as usize];
        for e in &edges {
            let x = e[0] - 1;
            let y = e[1] - 1;
            g[x as usize].push(y);
            g[y as usize].push(x);
        }
        fn dfs(g: &[Vec<i32>], root: i32, mask: &mut i32, d: &mut i32) -> i32 {
            let (mut first, mut second) = (0, 0);
            *mask &= !(1 << root);
            for &e in &g[root as usize] {
                if *mask & (1 << e) > 0 {
                    *mask &= !(1 << e);
                    let dist = 1 + dfs(g, e, mask, d);
                    if dist > first {
                        second = first;
                        first = dist;
                    } else if dist > second {
                        second = dist;
                    }
                }
            }
            *d = (*d).max(first + second);
            first
        }
        let mut result = vec![0; n as usize - 1];
        for i in 1i32..(1 << n) {
            let root = 32 - i.leading_zeros() as i32 - 1;
            let mut mask = i;
            let mut d = 0;
            dfs(&g, root, &mut mask, &mut d);
            if mask == 0 && d > 0 {
                result[d as usize - 1] += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 4;
        let edges = [[1, 2], [2, 3], [2, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(
            vec![3, 4, 0],
            Solution::count_subgraphs_for_each_diameter(n, edges)
        );
    }

    #[test]
    fn case2() {
        let n = 3;
        let edges = [[1, 2], [2, 3]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(
            vec![2, 1],
            Solution::count_subgraphs_for_each_diameter(n, edges)
        );
    }
}
