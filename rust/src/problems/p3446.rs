pub struct Solution;

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut diag = Vec::with_capacity(n);
        for d in -(n as i32 - 1)..n as i32 {
            diag.clear();
            let upper = d < 0;
            let (mut i, mut j) = (d.max(0), -d.min(0));
            while i < n as i32 && j < n as i32 {
                diag.push(grid[i as usize][j as usize]);
                i += 1;
                j += 1;
            }
            if upper {
                diag.sort_unstable();
            } else {
                diag.sort_unstable_by_key(|e| -e);
            }
            while i >= 1 && j >= 1 {
                i -= 1;
                j -= 1;
                grid[i as usize][j as usize] = diag.pop().unwrap();
            }
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 7, 3], [9, 8, 2], [4, 5, 6]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let expected = [[8, 2, 3], [9, 6, 7], [4, 5, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::sort_matrix(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 1], [1, 2]].iter().map(|r| r.to_vec()).collect();
        let expected = [[2, 1], [1, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::sort_matrix(grid));
    }
}
