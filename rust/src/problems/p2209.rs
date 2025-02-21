pub struct Solution;

impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        let floor = floor.as_bytes();
        let (m, n, len) = (floor.len(), num_carpets as usize, carpet_len as usize);
        let mut f = vec![-1; m];
        for (i, &t) in floor.iter().enumerate() {
            f[i] = if i > 0 { f[i - 1] } else { 0 } + i32::from(t == b'1');
        }
        for i in 1..=n {
            let mut g = vec![0; m];
            for j in len * i..m {
                g[j] = (g[j - 1] + i32::from(floor[j] == b'1')).min(f[j - len]);
            }
            f = g;
        }
        f[m - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::minimum_white_tiles("10110101".to_string(), 2, 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::minimum_white_tiles("11111".to_string(), 2, 3));
    }
}
