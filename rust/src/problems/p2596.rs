pub struct Solution;

fn is_valid_move((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> bool {
    let (dx, dy) = ((x1 - x2).abs(), (y1 - y2).abs());
    dx * dy == 2
}

impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        if grid[0][0] != 0 {
            return false;
        }
        let n = grid.len();
        let mut steps = vec![(0, 0); n * n];
        for (i, row) in grid.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                steps[c as usize] = (i as i32, j as i32);
            }
        }
        steps.windows(2).all(|w| is_valid_move(w[0], w[1]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [
            [0, 11, 16, 5, 20],
            [17, 4, 19, 10, 15],
            [12, 1, 8, 21, 6],
            [3, 18, 23, 14, 9],
            [24, 13, 2, 7, 22],
        ]
        .iter()
        .map(|row| row.to_vec())
        .collect();
        assert!(Solution::check_valid_grid(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 3, 6], [5, 8, 1], [2, 7, 4]]
            .iter()
            .map(|row| row.to_vec())
            .collect();
        assert!(!Solution::check_valid_grid(grid));
    }
}
