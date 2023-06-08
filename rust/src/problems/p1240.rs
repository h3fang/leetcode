pub struct Solution;

impl Solution {
    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        fn is_empty(r: &[Vec<bool>], i: usize, j: usize, k: usize) -> bool {
            (i..i + k).all(|i| (j..j + k).all(|j| !r[i][j]))
        }
        fn fill_up(r: &mut [Vec<bool>], i: usize, j: usize, k: usize, v: bool) {
            (i..i + k).for_each(|i| (j..j + k).for_each(|j| r[i][j] = v));
        }
        fn dfs(r: &mut [Vec<bool>], i: usize, j: usize, count: i32, result: &mut i32) {
            if count >= *result {
                return;
            }
            let (n, m) = (r.len(), r[0].len());
            if i >= n {
                *result = count;
            } else if j >= m {
                dfs(r, i + 1, 0, count, result);
            } else if r[i][j] {
                dfs(r, i, j + 1, count, result);
            } else {
                for k in (1..=(n - i).min(m - j)).rev() {
                    if !is_empty(r, i, j, k) {
                        continue;
                    }
                    fill_up(r, i, j, k, true);
                    dfs(r, i, j + k, count + 1, result);
                    fill_up(r, i, j, k, false);
                }
            }
        }
        let mut r = vec![vec![false; m as usize]; n as usize];
        let mut result = n.max(m);
        dfs(&mut r, 0, 0, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::tiling_rectangle(2, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::tiling_rectangle(5, 8));
    }

    #[test]
    fn case3() {
        assert_eq!(6, Solution::tiling_rectangle(11, 13));
    }
}
