pub struct Solution;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        if m < 3 || n < 3 {
            return 0;
        }
        let mut result = 0;
        for i in 0..m - 2 {
            for j in 0..n - 2 {
                if grid[i + 1][j + 1] != 5 {
                    continue;
                }
                let mut f = 0u16;
                let mut sum = [[0; 2]; 2];
                for (i, r) in grid[i..i + 3].iter().enumerate() {
                    for (j, c) in r[j..j + 3].iter().enumerate() {
                        f |= 1 << c;
                        if i < 2 {
                            sum[0][i] += c;
                        }
                        if j < 2 {
                            sum[1][j] += c;
                        }
                    }
                }
                if f == 0b11_1111_1110 && sum.iter().flatten().all(|&x| x == 15) {
                    result += 1;
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
        let grid = [[4, 3, 8, 4], [9, 5, 1, 9], [2, 7, 6, 2]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(1, Solution::num_magic_squares_inside(grid));
    }

    #[test]
    fn case2() {
        let grid = [[8]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::num_magic_squares_inside(grid));
    }
}
