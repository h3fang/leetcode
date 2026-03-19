pub struct Solution;

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let mut pre = vec![[0; 2]; grid[0].len()];
        let mut ans = 0;
        for row in grid {
            let mut sum = [0; 2];
            for (c, pre) in row.into_iter().zip(&mut pre) {
                if c != '.' {
                    pre[c as usize & 1] += 1;
                }
                sum[0] += pre[0];
                sum[1] += pre[1];
                if sum[0] > 0 && sum[0] == sum[1] {
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [["X", "Y", "."], ["Y", ".", "."]]
            .iter()
            .map(|row| row.iter().map(|c| c.as_bytes()[0] as char).collect())
            .collect();
        assert_eq!(3, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn case2() {
        let grid = [["X", "X"], ["X", "Y"]]
            .iter()
            .map(|row| row.iter().map(|c| c.as_bytes()[0] as char).collect())
            .collect();
        assert_eq!(0, Solution::number_of_submatrices(grid));
    }

    #[test]
    fn case3() {
        let grid = [[".", "."], [".", "."]]
            .iter()
            .map(|row| row.iter().map(|c| c.as_bytes()[0] as char).collect())
            .collect();
        assert_eq!(0, Solution::number_of_submatrices(grid));
    }
}
