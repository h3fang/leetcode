pub struct Solution;

impl Solution {
    pub fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
        fn factors(mut n: i32) -> (i32, i32) {
            let mut c2 = 0;
            let mut c5 = 0;
            while n % 2 == 0 {
                c2 += 1;
                n /= 2;
            }
            while n % 5 == 0 {
                c5 += 1;
                n /= 5;
            }
            (c2, c5)
        }

        let m = grid.len();
        let n = grid[0].len();

        let mut top = vec![vec![(0, 0); n + 1]; m + 1];
        let mut left = vec![vec![(0, 0); n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                let num = grid[i - 1][j - 1];
                let (c2, c5) = factors(num);

                let t = top[i - 1][j];
                top[i][j] = (t.0 + c2, t.1 + c5);

                let l = left[i][j - 1];
                left[i][j] = (l.0 + c2, l.1 + c5);
            }
        }

        let mut result = 0;
        for i in 1..=m {
            for j in 1..=n {
                let two = top[i][j].0 + left[i][j - 1].0;
                let five = top[i][j].1 + left[i][j - 1].1;
                result = result.max(two.min(five));

                let two = top[i][j].0 + left[i][n].0 - left[i][j].0;
                let five = top[i][j].1 + left[i][n].1 - left[i][j].1;
                result = result.max(two.min(five));

                let two = top[m][j].0 - top[i - 1][j].0 + left[i][j - 1].0;
                let five = top[m][j].1 - top[i - 1][j].1 + left[i][j - 1].1;
                result = result.max(two.min(five));

                let two = top[m][j].0 - top[i - 1][j].0 + left[i][n].0 - left[i][j].0;
                let five = top[m][j].1 - top[i - 1][j].1 + left[i][n].1 - left[i][j].1;
                result = result.max(two.min(five));
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [
            [23, 17, 15, 3, 20],
            [8, 1, 20, 27, 11],
            [9, 4, 6, 2, 21],
            [40, 9, 1, 10, 6],
            [22, 7, 4, 5, 3],
        ];
        let grid = grid.iter().map(|row| row.to_vec()).collect();
        assert_eq!(3, Solution::max_trailing_zeros(grid));
    }

    #[test]
    fn case2() {
        let grid = [[4, 3, 2], [7, 6, 1], [8, 8, 8]];
        let grid = grid.iter().map(|row| row.to_vec()).collect();
        assert_eq!(0, Solution::max_trailing_zeros(grid));
    }

    #[test]
    fn case3() {
        let grid = [[1, 5, 2, 4, 25]];
        let grid = grid.iter().map(|row| row.to_vec()).collect();
        assert_eq!(3, Solution::max_trailing_zeros(grid));
    }
}
