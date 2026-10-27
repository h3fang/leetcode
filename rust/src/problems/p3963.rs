pub struct Solution;

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let mut ans: Vec<String> = vec!["#".repeat(n as usize); m as usize];
        for (i, r) in ans.iter_mut().enumerate() {
            for (j, c) in unsafe { r.as_bytes_mut() }.iter_mut().enumerate() {
                if i == 0 || j == n as usize - 1 {
                    *c = b'.';
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(m: i32, n: i32, grid: &[String]) {
        let (m, n) = (m as usize, n as usize);
        assert_eq!(m, grid.len());
        assert_eq!(n, grid[0].len());
        assert_eq!(b'.', grid[0].as_bytes()[0]);
        assert_eq!(b'.', grid[m - 1].as_bytes()[n - 1]);
        let mut ways = 0;
        let mut q = Vec::with_capacity(m * n);
        q.push((0, 0));

        while let Some((i, j)) = q.pop() {
            if (i, j) == (m - 1, n - 1) {
                ways += 1;

                if ways > 1 {
                    break;
                }
            }

            for (i1, j1) in [(i, j + 1), (i + 1, j)] {
                if i1 == m || j1 == n || grid[i1].as_bytes()[j1] == b'#' {
                    continue;
                }
                q.push((i1, j1));
            }
        }

        assert!(ways == 1);
    }

    #[test]
    fn case1() {
        let (m, n) = (2, 3);
        let grid = Solution::create_grid(m, n);
        check(m, n, &grid);
    }

    #[test]
    fn case2() {
        let (m, n) = (3, 3);
        let grid = Solution::create_grid(m, n);
        check(m, n, &grid);
    }

    #[test]
    fn case3() {
        let (m, n) = (1, 4);
        let grid = Solution::create_grid(m, n);
        check(m, n, &grid);
    }
}
