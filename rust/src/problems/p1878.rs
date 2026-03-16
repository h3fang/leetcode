pub struct Solution;

fn diag_sum(presum: &[Vec<i32>], i: usize, j: usize, k: usize) -> i32 {
    presum[i + k][j + k] - presum[i][j]
}

fn rdiag_sum(presum: &[Vec<i32>], i: usize, j: usize, k: usize) -> i32 {
    presum[i + k][j + 1 - k] - presum[i][j + 1]
}

fn update(f: &mut [i32], v: i32) {
    if v > f[0] {
        f[2] = f[1];
        f[1] = f[0];
        f[0] = v;
    } else if v < f[0] && v > f[1] {
        f[2] = f[1];
        f[1] = v;
    } else if v < f[1] && v > f[2] {
        f[2] = v;
    }
}

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());

        let mut diag = vec![vec![0; n + 1]; m + 1];
        let mut rdiag = vec![vec![0; n + 1]; m + 1];

        for (i, r) in grid.iter().enumerate() {
            for (j, &v) in r.iter().enumerate() {
                diag[i + 1][j + 1] = diag[i][j] + v;
                rdiag[i + 1][j] = rdiag[i][j + 1] + v;
            }
        }

        let mut f = vec![0; 3];

        for (i, r) in grid.iter().enumerate() {
            for (j, &v) in r.iter().enumerate() {
                update(&mut f, v);
                let max = i.min(m - 1 - i).min(j).min(n - 1 - j);
                for k in 1..=max {
                    let len = diag_sum(&diag, i - k, j, k)
                        + diag_sum(&diag, i, j - k, k)
                        + rdiag_sum(&rdiag, i + 1 - k, j - 1, k - 1)
                        + rdiag_sum(&rdiag, i, j + k, k + 1);
                    update(&mut f, len);
                }
            }
        }

        while let Some(&0) = f.last() {
            f.pop();
        }

        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [
            [3, 4, 5, 1, 3],
            [3, 3, 4, 2, 3],
            [20, 30, 200, 40, 10],
            [1, 5, 5, 4, 1],
            [4, 3, 2, 2, 5],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(vec![228, 216, 211], Solution::get_biggest_three(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(vec![20, 9, 8], Solution::get_biggest_three(grid));
    }

    #[test]
    fn case3() {
        let grid = [[7, 7, 7]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![7], Solution::get_biggest_three(grid));
    }
}
