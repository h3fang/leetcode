pub struct Solution;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let (m, n) = (key.len(), ring.len());
        let mut p = vec![vec![]; 26];
        let (key, ring) = (key.as_bytes(), ring.as_bytes());
        for (i, &x) in ring.iter().enumerate() {
            p[(x - b'a') as usize].push(i);
        }
        let mut f = vec![vec![i32::MAX / 2; n]; m];
        for &i in &p[(key[0] - b'a') as usize] {
            f[0][i] = 1 + i.min(n - i) as i32;
        }
        for (i, c) in key.iter().enumerate().skip(1) {
            for &j in &p[(c - b'a') as usize] {
                for &k in &p[(key[i - 1] - b'a') as usize] {
                    f[i][j] =
                        f[i][j].min(f[i - 1][k] + 1 + j.abs_diff(k).min(n - j.abs_diff(k)) as i32);
                }
            }
        }
        *f[m - 1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            4,
            Solution::find_rotate_steps("godding".to_string(), "gd".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            13,
            Solution::find_rotate_steps("godding".to_string(), "godding".to_string())
        );
    }
}
