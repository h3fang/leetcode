pub struct Solution;

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut d = vec![vec![0; n + 2]; n + 2];
        for q in queries {
            let &[r1, c1, r2, c2, ..] = q.as_slice() else {
                unreachable!();
            };
            d[r1 as usize + 1][c1 as usize + 1] += 1;
            d[r1 as usize + 1][c2 as usize + 2] -= 1;
            d[r2 as usize + 2][c1 as usize + 1] -= 1;
            d[r2 as usize + 2][c2 as usize + 2] += 1;
        }
        let mut ans = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                d[i + 1][j + 1] += d[i + 1][j] + d[i][j + 1] - d[i][j];
                ans[i][j] = d[i + 1][j + 1];
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let queries = [[1, 1, 2, 2], [0, 0, 1, 1]]
            .iter()
            .map(|q| q.to_vec())
            .collect();
        let expected = [[1, 1, 0], [1, 2, 1], [0, 1, 1]]
            .iter()
            .map(|q| q.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::range_add_queries(n, queries));
    }

    #[test]
    fn case2() {
        let n = 2;
        let queries = [[0, 0, 1, 1]].iter().map(|q| q.to_vec()).collect();
        let expected = [[1, 1], [1, 1]]
            .iter()
            .map(|q| q.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::range_add_queries(n, queries));
    }
}
