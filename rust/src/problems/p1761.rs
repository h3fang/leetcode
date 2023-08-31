pub struct Solution;

impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![false; n]; n];
        let mut h = vec![vec![]; n];
        let mut degree = vec![0; n];

        for e in &edges {
            let (x, y) = (e[0] as usize - 1, e[1] as usize - 1);
            g[x][y] = true;
            g[y][x] = true;
            degree[x] += 1;
            degree[y] += 1;
        }

        for e in &edges {
            let (x, y) = (e[0] as usize - 1, e[1] as usize - 1);
            if degree[x] < degree[y] || (degree[x] == degree[y] && x < y) {
                h[x].push(y);
            } else {
                h[y].push(x);
            }
        }
        let mut result = i32::MAX;
        for i in 0..n {
            for &j in &h[i] {
                for &k in &h[j] {
                    if g[i][k] {
                        result = result.min(degree[i] + degree[j] + degree[k] - 6);
                    }
                }
            }
        }
        if result == i32::MAX {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[1, 2], [1, 3], [3, 2], [4, 1], [5, 2], [3, 6]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(3, Solution::min_trio_degree(6, edges));
    }

    #[test]
    fn case2() {
        let edges = [
            [1, 3],
            [4, 1],
            [4, 3],
            [2, 5],
            [5, 6],
            [6, 7],
            [7, 5],
            [2, 6],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        assert_eq!(0, Solution::min_trio_degree(7, edges));
    }
}
