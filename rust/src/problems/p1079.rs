pub struct Solution;

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut nck = [[0; 8]; 8];
        (0..8).for_each(|i| nck[i][0] = 1);
        for i in 1..8 {
            for j in 1..8 {
                nck[i][j] = nck[i - 1][j - 1] + nck[i - 1][j];
            }
        }
        let mut freq = [0; 26];
        for &c in tiles.as_bytes() {
            freq[(c - b'A') as usize] += 1;
        }
        let freq = freq.into_iter().filter(|&f| f > 0).collect::<Vec<_>>();
        let m = freq.len();
        let n = tiles.len();
        let mut f = vec![vec![0; n + 1]; m + 1];
        f[0][0] = 1;
        for (i, c) in freq.into_iter().enumerate() {
            for (j, nck) in nck.iter().enumerate().take(n + 1) {
                for (k, x) in nck.iter().enumerate().take(j.min(c) + 1) {
                    f[i + 1][j] += f[i][j - k] * x;
                }
            }
        }
        f[m].iter().sum::<i32>() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::num_tile_possibilities("AAB".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(188, Solution::num_tile_possibilities("AAABBC".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::num_tile_possibilities("V".to_string()));
    }
}
