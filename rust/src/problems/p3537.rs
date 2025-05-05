pub struct Solution;

fn dfs(ans: &mut [Vec<i32>], i: usize, j: usize, m: usize, x: i32) {
    if m == 1 {
        ans[i][j] = x;
        return;
    }
    let n = m / 2;
    let cnt = n as i32 * n as i32;
    dfs(ans, i, j + n, n, x);
    dfs(ans, i + n, j + n, n, x + cnt);
    dfs(ans, i + n, j, n, x + cnt * 2);
    dfs(ans, i, j, n, x + cnt * 3);
}

impl Solution {
    pub fn special_grid(n: i32) -> Vec<Vec<i32>> {
        let m = 1 << n;
        let mut ans = vec![vec![0; m]; m];
        dfs(&mut ans, 0, 0, m, 0);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = vec![vec![0]];
        assert_eq!(expected, Solution::special_grid(0));
    }

    #[test]
    fn case2() {
        let expected = vec![vec![3, 0], vec![2, 1]];
        assert_eq!(expected, Solution::special_grid(1));
    }

    #[test]
    fn case3() {
        let expected = vec![
            vec![15, 12, 3, 0],
            vec![14, 13, 2, 1],
            vec![11, 8, 7, 4],
            vec![10, 9, 6, 5],
        ];
        assert_eq!(expected, Solution::special_grid(2));
    }
}
