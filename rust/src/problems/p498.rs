pub struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len() as i32;
        let n = mat[0].len() as i32;
        let mut result = Vec::with_capacity((m * n) as usize);
        let mut i = 0i32;
        let mut j = 0i32;
        let mut top_right = true;
        loop {
            if i >= 0 && j >= 0 && i < m && j < n {
                result.push(mat[i as usize][j as usize]);
                if i == m - 1 && j == n - 1 {
                    break;
                }
            }
            if top_right {
                i -= 1;
                j += 1;
                if i < 0 || j == n {
                    top_right = false;
                    i += 1;
                }
            } else {
                i += 1;
                j -= 1;
                if i >= m || j < 0 {
                    top_right = true;
                    j += 1;
                }
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
        let mat = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let mat = mat.iter().map(|r| r.to_vec()).collect();
        assert_eq!(
            vec![1, 2, 4, 7, 5, 3, 6, 8, 9],
            Solution::find_diagonal_order(mat)
        );
    }
}
