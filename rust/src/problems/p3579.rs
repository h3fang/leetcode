pub struct Solution;

fn index(a: u8, b: u8) -> usize {
    ((a - b'a') as usize) * 26 + (b - b'a') as usize
}

fn update(cnt: &mut [i32], a: u8, b: u8, ops: &mut i32) {
    if a != b {
        if cnt[index(b, a)] > 0 {
            cnt[index(b, a)] -= 1;
        } else {
            cnt[index(a, b)] += 1;
            *ops += 1;
        }
    }
}

impl Solution {
    pub fn min_operations(word1: String, word2: String) -> i32 {
        let n = word1.len();
        let (w1, w2) = (word1.as_bytes(), word2.as_bytes());
        let mut rev = vec![vec![0; n]; n];
        for i in 0..2 * n as i32 - 1 {
            let mut cnt = [0; 26 * 26];
            let mut ops = 1;
            let (mut l, mut r) = (i / 2, (i + 1) / 2);
            while l >= 0 && r < n as i32 {
                let (a, b) = (w1[l as usize], w2[r as usize]);
                update(&mut cnt, a, b, &mut ops);
                if l != r {
                    let (a, b) = (w1[r as usize], w2[l as usize]);
                    update(&mut cnt, a, b, &mut ops);
                }
                rev[l as usize][r as usize] = ops;
                l -= 1;
                r += 1;
            }
        }

        let mut f = vec![0; n + 1];
        for i in 0..n {
            let mut min = n as i32;
            let mut cnt = [0; 26 * 26];
            let mut ops = 0;
            for j in (0..=i).rev() {
                let (a, b) = (w1[j], w2[j]);
                update(&mut cnt, a, b, &mut ops);
                min = min.min(f[j] + ops.min(rev[j][i]));
            }
            f[i + 1] = min;
        }
        f[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            4,
            Solution::min_operations("abcdf".to_string(), "dacbe".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            4,
            Solution::min_operations("abceded".to_string(), "baecfef".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            2,
            Solution::min_operations("abcdef".to_string(), "fedabc".to_string())
        );
    }
}
