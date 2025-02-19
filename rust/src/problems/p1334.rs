pub struct Solution;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![i32::MAX / 2; n]; n];
        for e in edges {
            g[e[0] as usize][e[1] as usize] = e[2];
            g[e[1] as usize][e[0] as usize] = e[2];
        }
        (0..n).for_each(|i| g[i][i] = 0);
        let (mut min, mut result) = (i32::MAX, 0);
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
                }
            }
        }
        for (i, row) in g.iter().enumerate() {
            let c = row.iter().filter(|&&x| x <= distance_threshold).count() as i32;
            if c <= min {
                min = c;
                result = i as i32;
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
        let edges = [[0, 1, 3], [1, 2, 1], [1, 3, 4], [2, 3, 1]]
            .iter()
            .map(|e| e.to_vec())
            .collect();
        assert_eq!(3, Solution::find_the_city(4, edges, 4));
    }

    #[test]
    fn case2() {
        let edges = [
            [0, 1, 2],
            [0, 4, 8],
            [1, 2, 3],
            [1, 4, 2],
            [2, 3, 1],
            [3, 4, 1],
        ]
        .iter()
        .map(|e| e.to_vec())
        .collect();
        assert_eq!(0, Solution::find_the_city(5, edges, 2));
    }
}
