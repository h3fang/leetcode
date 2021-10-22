pub struct Solution;

impl Solution {
    pub fn find_circle_num(mut is_connected: Vec<Vec<i32>>) -> i32 {
        fn dfs(is_connected: &mut Vec<Vec<i32>>, i: usize) {
            for j in 0..is_connected[i].len() {
                if is_connected[i][j] == 1 {
                    is_connected[i][j] = 0;
                    is_connected[j][i] = 0;
                    dfs(is_connected, j);
                }
            }
        }

        let mut m = 0;
        for i in 0..is_connected.len() {
            if is_connected[i][i] == 1 {
                m += 1;
                dfs(&mut is_connected, i);
            }
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        assert_eq!(2, Solution::find_circle_num(is_connected));
    }

    #[test]
    fn case2() {
        let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(3, Solution::find_circle_num(is_connected));
    }
}
