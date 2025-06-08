pub struct Solution;

fn index(a: u8, b: u8) -> usize {
    ((a - b'a') as usize) * 26 + (b - b'a') as usize
}

impl Solution {
    pub fn min_operations(word1: String, word2: String) -> i32 {
        let n = word1.len();
        let (w1, w2) = (word1.as_bytes(), word2.as_bytes());
        let mut f = vec![0; n + 1];
        for i in 0..n {
            let mut cnt = [0; 26 * 26];
            let (mut min, mut ops) = (n as i32, 0);
            for j in (0..=i).rev() {
                let (a, b) = (w1[j], w2[j]);
                if a != b {
                    if cnt[index(b, a)] > 0 {
                        cnt[index(b, a)] -= 1;
                    } else {
                        cnt[index(a, b)] += 1;
                        ops += 1;
                    }
                }

                let mut cnt_rev = [0; 26 * 26];
                let mut ops_rev = 1;
                for p in j..=i {
                    let (a, b) = (w1[p], w2[i + j - p]);
                    if a != b {
                        if cnt_rev[index(b, a)] > 0 {
                            cnt_rev[index(b, a)] -= 1;
                        } else {
                            cnt_rev[index(a, b)] += 1;
                            ops_rev += 1;
                        }
                    }
                }
                min = min.min(f[j] + ops.min(ops_rev));
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
