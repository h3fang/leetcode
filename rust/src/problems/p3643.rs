pub struct Solution;

impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let (x, y, k) = (x as usize, y as usize, k as usize);
        for i in x..x + k / 2 {
            if let Ok([a, b]) = grid.get_disjoint_mut([i, x * 2 + k - i - 1]) {
                a[y..y + k].swap_with_slice(&mut b[y..y + k]);
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
        let grid = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        let expected = [
            [1, 2, 3, 4],
            [13, 14, 15, 8],
            [9, 10, 11, 12],
            [5, 6, 7, 16],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect::<Vec<_>>();
        assert_eq!(expected, Solution::reverse_submatrix(grid, 1, 0, 3));
    }

    #[test]
    fn case2() {
        let grid = [[3, 4, 2, 3], [2, 3, 4, 2]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let expected = [[3, 4, 4, 2], [2, 3, 2, 3]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::reverse_submatrix(grid, 0, 2, 2));
    }
}
