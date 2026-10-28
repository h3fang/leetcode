pub struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut f = vec![[false; 1024]; n];
        f[0][0] = true;

        for r in grid {
            for (j, &c) in r.iter().enumerate() {
                let mut next = [false; 1024];
                for (k, &a) in f[j].iter().enumerate() {
                    if a || (j > 0 && f[j - 1][k]) {
                        next[k ^ c as usize] = true;
                    }
                }
                f[j] = next;
            }
        }

        f[n - 1].iter().position(|&e| e).unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 2], [3, 4]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(6, Solution::min_cost(grid));
    }

    #[test]
    fn case2() {
        let grid = [[6, 7], [5, 8]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(9, Solution::min_cost(grid));
    }

    #[test]
    fn case3() {
        let grid = [[2, 7, 5]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::min_cost(grid));
    }
}
