pub struct Solution;

impl Solution {
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        let (n, goal) = (n as usize, goal as usize);
        let mut f = vec![vec![0; n + 1]; goal + 1];
        f[0][0] = 1;
        for i in 1..=goal {
            for j in 1..=n {
                f[i][j] += f[i - 1][j - 1] * (n + 1 - j);
                f[i][j] += f[i - 1][j] * ((j as i32 - k).max(0) as usize);
                f[i][j] %= 10_0000_0007;
            }
        }
        f[goal][n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::num_music_playlists(3, 3, 1));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::num_music_playlists(2, 3, 0));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::num_music_playlists(2, 3, 1));
    }
}
