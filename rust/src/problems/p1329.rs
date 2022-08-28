pub struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop, clippy::manual_memcpy)]
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        for i in 0..m {
            let mut d = Vec::with_capacity(m.max(n));
            for k in 0..n.min(m - i) {
                d.push(mat[i + k][k]);
            }
            d.sort_unstable();
            for k in 0..n.min(m - i) {
                mat[i + k][k] = d[k];
            }
        }

        for j in 0..n {
            let mut d = Vec::with_capacity(m.max(n));
            for k in 0..m.min(n - j) {
                d.push(mat[k][j + k]);
            }
            d.sort_unstable();
            for k in 0..m.min(n - j) {
                mat[k][j + k] = d[k];
            }
        }
        mat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = [[3, 3, 1, 1], [2, 2, 1, 2], [1, 1, 1, 2]];
        let mat = mat.iter().map(|r| r.to_vec()).collect();
        let expected = [[1, 1, 1, 1], [1, 2, 2, 2], [1, 2, 3, 3]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::diagonal_sort(mat));
    }

    #[test]
    fn case2() {
        let mat = [
            [11, 25, 66, 1, 69, 7],
            [23, 55, 17, 45, 15, 52],
            [75, 31, 36, 44, 58, 8],
            [22, 27, 33, 25, 68, 4],
            [84, 28, 14, 11, 5, 50],
        ];
        let mat = mat.iter().map(|r| r.to_vec()).collect();
        let expected = [
            [5, 17, 4, 1, 52, 7],
            [11, 11, 25, 45, 8, 69],
            [14, 23, 25, 44, 58, 15],
            [22, 27, 31, 36, 50, 66],
            [84, 28, 75, 33, 55, 68],
        ];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::diagonal_sort(mat));
    }
}
