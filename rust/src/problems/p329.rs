pub struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        fn dfs(matrix: &[Vec<i32>], (i, j): (i32, i32), cache: &mut [Vec<i32>]) -> i32 {
            if cache[i as usize][j as usize] >= 0 {
                return cache[i as usize][j as usize];
            }
            let mut result = 0;
            let curr = matrix[i as usize][j as usize];
            for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                if i1 < 0 || j1 < 0 || i1 == matrix.len() as i32 || j1 == matrix[0].len() as i32 {
                    continue;
                }
                if matrix[i1 as usize][j1 as usize] > curr {
                    result = result.max(dfs(matrix, (i1, j1), cache));
                }
            }
            result += 1;
            cache[i as usize][j as usize] = result;
            result
        }

        let m = matrix.len();
        let n = matrix[0].len();
        let mut cache = vec![vec![-1; n]; m];
        let mut result = 0;
        for i in 0..m {
            for j in 0..n {
                let r = dfs(&matrix, (i as i32, j as i32), &mut cache);
                result = result.max(r);
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
        let matrix = [[9, 9, 4], [6, 6, 8], [2, 1, 1]];
        let matrix = matrix.iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::longest_increasing_path(matrix));
    }

    #[test]
    fn case2() {
        let matrix = [[3, 4, 5], [3, 2, 6], [2, 2, 1]];
        let matrix = matrix.iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::longest_increasing_path(matrix));
    }
}
