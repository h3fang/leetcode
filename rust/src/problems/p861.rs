pub struct Solution;

impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        grid.iter_mut().for_each(|r| {
            if r[0] == 0 {
                r.iter_mut().for_each(|x| *x = 1 - *x);
            }
        });
        for i in 1..n {
            let ones = grid.iter().filter(|&r| r[i] == 1).count();
            if ones * 2 < m {
                grid.iter_mut().for_each(|r| r[i] = 1 - r[i]);
            }
        }
        grid.into_iter()
            .map(|r| r.iter().fold(0, |acc, x| (acc << 1) + x))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 0, 1, 1], [1, 0, 1, 0], [1, 1, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(39, Solution::matrix_score(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::matrix_score(grid));
    }
}
