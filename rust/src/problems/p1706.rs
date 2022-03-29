pub struct Solution;

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid[0].len() as i32;
        let mut result = Vec::with_capacity(n as usize);
        for col in 0..n {
            let mut c = col;
            for row in &grid {
                let dir = row[c as usize];
                c += dir;
                if c < 0 || c == n || row[c as usize] != dir {
                    c = -1;
                    break;
                }
            }
            result.push(c);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [
            [1, 1, 1, -1, -1],
            [1, 1, 1, -1, -1],
            [-1, -1, -1, 1, 1],
            [1, 1, 1, 1, -1],
            [-1, -1, -1, -1, -1],
        ];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![1, -1, -1, -1, -1], Solution::find_ball(grid));
    }

    #[test]
    fn case2() {
        let grid = [[-1]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![-1], Solution::find_ball(grid));
    }

    #[test]
    fn case3() {
        let grid = [
            [1, 1, 1, 1, 1, 1],
            [-1, -1, -1, -1, -1, -1],
            [1, 1, 1, 1, 1, 1],
            [-1, -1, -1, -1, -1, -1],
        ];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![0, 1, 2, 3, 4, -1], Solution::find_ball(grid));
    }
}
