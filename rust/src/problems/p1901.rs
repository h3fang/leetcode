pub struct Solution;

impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let (mut l, mut r) = (0, m - 1);
        while l <= r {
            let i = l + (r - l) / 2;
            let j = mat[i]
                .iter()
                .enumerate()
                .fold((i32::MIN, 0), |(mut max, mut idx), (j, &e)| {
                    if e >= max {
                        max = e;
                        idx = j;
                    }
                    (max, idx)
                })
                .1;
            if i > 0 && mat[i][j] < mat[i - 1][j] {
                r = i - 1;
            } else if i + 1 < m && mat[i][j] < mat[i + 1][j] {
                l = i + 1;
            } else {
                return vec![i as i32, j as i32];
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(mat: Vec<Vec<i32>>) {
        let (m, n) = (mat.len() as i32, mat[0].len() as i32);
        let r = Solution::find_peak_grid(mat.clone());
        let (i, j) = (r[0], r[1]);
        for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            assert!(
                i1 < 0
                    || j1 < 0
                    || i1 == m
                    || j1 == n
                    || mat[i as usize][j as usize] > mat[i1 as usize][j1 as usize]
            );
        }
    }

    #[test]
    fn case1() {
        check(
            [[1, 4], [3, 2]]
                .iter()
                .map(|r| r.to_vec())
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn case2() {
        check(
            [[10, 20, 15], [21, 30, 14], [7, 16, 32]]
                .iter()
                .map(|r| r.to_vec())
                .collect(),
        );
    }
}
