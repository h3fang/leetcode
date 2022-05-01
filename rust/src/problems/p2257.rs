pub struct Solution;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut grid = vec![vec![true; n as usize]; m as usize];
        let mut occu = vec![vec![false; n as usize]; m as usize];
        for e in &guards {
            occu[e[0] as usize][e[1] as usize] = true;
        }

        for e in &walls {
            occu[e[0] as usize][e[1] as usize] |= true;
            grid[e[0] as usize][e[1] as usize] = false;
        }

        for e in &guards {
            let i0 = e[0] as usize;
            let j0 = e[1] as usize;
            grid[i0][j0] = false;
            for i in (0..i0).rev() {
                grid[i][j0] = false;
                if occu[i][j0] {
                    break;
                }
            }

            for i in i0 + 1..m as usize {
                grid[i][j0] = false;
                if occu[i][j0] {
                    break;
                }
            }

            for j in (0..j0).rev() {
                grid[i0][j] = false;
                if occu[i0][j] {
                    break;
                }
            }

            for j in j0 + 1..n as usize {
                grid[i0][j] = false;
                if occu[i0][j] {
                    break;
                }
            }
        }
        grid.iter().flatten().filter(|c| **c).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_cells(cells: &[[i32; 2]]) -> Vec<Vec<i32>> {
        cells.iter().map(|c| c.to_vec()).collect()
    }

    #[test]
    fn case1() {
        let m = 4;
        let n = 6;
        let guards = parse_cells(&[[0, 0], [1, 1], [2, 3]]);
        let walls = parse_cells(&[[0, 1], [2, 2], [1, 4]]);
        assert_eq!(7, Solution::count_unguarded(m, n, guards, walls));
    }

    #[test]
    fn case2() {
        let m = 3;
        let n = 3;
        let guards = parse_cells(&[[1, 1]]);
        let walls = parse_cells(&[[0, 1], [1, 0], [2, 1], [1, 2]]);
        assert_eq!(4, Solution::count_unguarded(m, n, guards, walls));
    }

    #[test]
    fn case3() {
        let m = 3;
        let n = 4;
        let guards = parse_cells(&[[0, 1], [1, 2], [2, 0], [2, 1]]);
        let walls = parse_cells(&[[1, 3], [0, 3], [1, 0], [2, 2], [0, 2]]);
        assert_eq!(1, Solution::count_unguarded(m, n, guards, walls));
    }
}
