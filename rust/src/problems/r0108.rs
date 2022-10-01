pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut rows = vec![false; m];
        let mut cols = vec![false; n];
        for (i, r) in matrix.iter().enumerate() {
            for (j, e) in r.iter().enumerate() {
                if *e == 0 {
                    rows[i] = true;
                    cols[j] = true;
                }
            }
        }

        for (i, r) in rows.into_iter().enumerate() {
            if r {
                matrix[i].iter_mut().for_each(|e| *e = 0);
            }
        }

        for (j, c) in cols.into_iter().enumerate() {
            if c {
                (0..m).for_each(|i| matrix[i][j] = 0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut matrix = [[1, 1, 1], [1, 0, 1], [1, 1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let expected = [[1, 0, 1], [0, 0, 0], [1, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        Solution::set_zeroes(&mut matrix);
        assert_eq!(expected, matrix);
    }
}
