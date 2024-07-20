pub struct Solution;

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut more = Vec::with_capacity(4);
        let mut less = Vec::with_capacity(8);
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c > 1 {
                    for _ in 0..c - 1 {
                        more.push((i, j));
                    }
                } else if c == 0 {
                    less.push((i, j));
                }
            }
        }

        fn dfs(
            more: &mut Vec<(usize, usize)>,
            less: &[(usize, usize)],
            i: usize,
            result: &mut i32,
        ) {
            if i == more.len() - 1 {
                let sum = more
                    .iter()
                    .zip(less)
                    .map(|(&(i1, j1), &(i2, j2))| i1.abs_diff(i2) + j1.abs_diff(j2))
                    .sum::<usize>() as i32;
                if sum < *result {
                    *result = sum;
                }
                return;
            }
            for j in i..more.len() {
                more.swap(i, j);
                dfs(more, less, i + 1, result);
                more.swap(i, j);
            }
        }
        let mut result = i32::MAX;
        dfs(&mut more, &less, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 1, 0], [1, 1, 1], [1, 2, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(3, Solution::minimum_moves(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 3, 0], [1, 0, 0], [1, 0, 3]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(4, Solution::minimum_moves(grid));
    }
}
