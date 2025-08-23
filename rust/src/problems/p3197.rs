pub struct Solution;

impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        solve(&grid).min(solve(&rotate(&grid)))
    }
}

fn rotate(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut ans = vec![vec![0; m]; n];
    for (i, r) in grid.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            ans[n - j - 1][i] = c;
        }
    }
    ans
}

fn min_area(grid: &[Vec<i32>], lb: usize, rb: usize) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let (mut l, mut r, mut t, mut b) = (n as i32, -1, m as i32, -1);
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row[lb..rb].iter().enumerate() {
            if c == 0 {
                continue;
            }
            l = l.min(j as i32);
            r = r.max(j as i32);
            t = t.min(i as i32);
            b = b.max(i as i32);
        }
    }
    if t <= b {
        (r - l + 1) * (b - t + 1)
    } else {
        i32::MAX / 3
    }
}

fn solve(grid: &[Vec<i32>]) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut ans = (m * n) as i32;

    for i in 1..m {
        for j in 1..n {
            ans = ans.min(
                min_area(&grid[0..i], 0, j)
                    + min_area(&grid[0..i], j, n)
                    + min_area(&grid[i..], 0, n),
            );

            ans = ans.min(
                min_area(&grid[0..i], 0, n)
                    + min_area(&grid[i..], 0, j)
                    + min_area(&grid[i..], j, n),
            );
        }
    }

    for i in 1..m - 1 {
        for j in i + 1..m {
            ans = ans.min(
                min_area(&grid[0..i], 0, n)
                    + min_area(&grid[i..j], 0, n)
                    + min_area(&grid[j..], 0, n),
            );
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 0, 1], [1, 1, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(5, Solution::minimum_sum(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 0, 1, 0], [0, 1, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(5, Solution::minimum_sum(grid));
    }

    #[test]
    fn case3() {
        let grid = [[0, 0, 0], [0, 0, 0], [1, 1, 0], [0, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(3, Solution::minimum_sum(grid));
    }
}
