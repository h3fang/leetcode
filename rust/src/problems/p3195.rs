pub struct Solution;

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut l, mut r, mut t, mut b) = (n as i32, -1, m as i32, -1);
        for (i, row) in grid.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == 0 {
                    continue;
                }
                l = l.min(j as i32);
                r = r.max(j as i32);
                t = t.min(i as i32);
                b = b.max(i as i32);
            }
        }
        (r - l + 1) * (b - t + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 1, 0], [1, 0, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(6, Solution::minimum_area(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 0], [1, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::minimum_area(grid));
    }
}
