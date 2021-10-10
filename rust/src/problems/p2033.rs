pub struct Solution;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut sum: i64 = 0;
        for &i in grid.iter().flatten() {
            min = min.min(i);
            max = max.max(i);
            sum += i as i64;
        }

        for &i in grid.iter().flatten() {
            if (i - min) % x != 0 {
                return -1;
            }
        }

        let avg = (sum / (grid.len() * grid[0].len()) as i64) as i32;
        let avg = avg - (avg - min) % x;

        let mut r = i32::MAX;

        for p in (avg..=min).rev().step_by(x as usize) {
            let c: i32 = grid.iter().flatten().map(|&i| (i - p).abs() / x).sum();
            if c > r {
                break;
            }
            r = c;
        }

        for p in (avg + x..=max).step_by(x as usize) {
            let c: i32 = grid.iter().flatten().map(|&i| (i - p).abs() / x).sum();
            if c > r {
                break;
            }
            r = c;
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[2, 4], [6, 8]];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(4, Solution::min_operations(grid, 2));
    }

    #[test]
    fn case2() {
        let grid = [[1, 5], [2, 3]];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(5, Solution::min_operations(grid, 1));
    }

    #[test]
    fn case3() {
        let grid = [[1, 2], [3, 4]];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(-1, Solution::min_operations(grid, 2));
    }

    #[test]
    fn case4() {
        let grid = [[931, 128], [639, 712]];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(12, Solution::min_operations(grid, 73));
    }

    #[test]
    fn case5() {
        let grid = [
            [596, 904, 960, 232, 120, 932, 176],
            [372, 792, 288, 848, 960, 960, 764],
            [652, 92, 904, 120, 680, 904, 120],
            [372, 960, 92, 680, 876, 624, 904],
            [176, 652, 64, 344, 316, 764, 316],
            [820, 624, 848, 596, 960, 960, 372],
            [708, 120, 456, 92, 484, 932, 540],
        ];
        let grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(473, Solution::min_operations(grid, 28));
    }
}
