pub struct Solution;

impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let (mut rows, mut cols) = (vec![0; grid.len()], vec![0; grid[0].len()]);
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 1 {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }
        let mut result = 0;
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 1 {
                    result += (rows[i] - 1) * (cols[j] - 1);
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
        let grid = [[0, 1, 0], [0, 1, 1], [0, 1, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(2, Solution::number_of_right_triangles(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 0, 0, 0], [0, 1, 0, 1], [1, 0, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(0, Solution::number_of_right_triangles(grid));
    }

    #[test]
    fn case3() {
        let grid = [[1, 0, 1], [1, 0, 0], [1, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(2, Solution::number_of_right_triangles(grid));
    }
}
