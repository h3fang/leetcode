pub struct Solution;

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut zeros = grid
            .iter()
            .map(|r| {
                r.iter()
                    .rposition(|x| *x == 1)
                    .map(|i| n - 1 - i)
                    .unwrap_or(n)
            })
            .collect::<Vec<_>>();

        let mut ans = 0;
        for i in 0..zeros.len() - 1 {
            let z = n - 1 - i;
            let Some(j) = zeros.iter().skip(i).position(|y| *y >= z) else {
                return -1;
            };
            zeros.copy_within(i..i + j, i + 1);
            ans += j as i32;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 0, 1], [1, 1, 0], [1, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(3, Solution::min_swaps(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 1, 1, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 1, 1, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(-1, Solution::min_swaps(grid));
    }

    #[test]
    fn case3() {
        let grid = [[1, 0, 0], [1, 1, 0], [1, 1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(0, Solution::min_swaps(grid));
    }
}
