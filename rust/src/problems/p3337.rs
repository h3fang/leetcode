pub struct Solution;

const MOD: i64 = 10_0000_0007;

fn mul(a: &[[i64; 26]; 26], b: &[[i64; 26]; 26]) -> [[i64; 26]; 26] {
    let mut ans = [[0; 26]; 26];
    for (i, r) in a.iter().enumerate() {
        for (k, &a) in r.iter().enumerate() {
            if a == 0 {
                continue;
            }
            for j in 0..b[0].len() {
                ans[i][j] = (ans[i][j] + a * b[k][j]) % MOD;
            }
        }
    }
    ans
}

fn pow(mut a: [[i64; 26]; 26], mut t: i32) -> [[i64; 26]; 26] {
    let mut ans = [[0; 26]; 26];
    for (i, r) in ans.iter_mut().enumerate() {
        r[i] = 1;
    }
    while t > 0 {
        if t & 1 == 1 {
            ans = mul(&ans, &a);
        }
        a = mul(&a, &a);
        t /= 2;
    }
    ans
}

impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let mut f = [0; 26];
        for b in s.bytes() {
            f[(b - b'a') as usize] += 1;
        }
        let mut a = [[0; 26]; 26];
        for (i, c) in nums.into_iter().enumerate() {
            for j in i + 1..i + 1 + c as usize {
                a[i][j % 26] = 1;
            }
        }
        let a = pow(a, t);
        let mut ans = 0;
        for i in 0..26 {
            for j in 0..26 {
                ans = (ans + a[j][i] * f[j]) % MOD;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            7,
            Solution::length_after_transformations(
                "abcyy".to_string(),
                2,
                vec![
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2
                ]
            )
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            8,
            Solution::length_after_transformations(
                "azbk".to_string(),
                1,
                vec![
                    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2
                ]
            )
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            2,
            Solution::length_after_transformations(
                "k".to_string(),
                2,
                vec![
                    2, 2, 1, 3, 1, 1, 2, 3, 3, 2, 1, 2, 2, 1, 1, 3, 1, 2, 2, 1, 3, 3, 3, 2, 2, 1
                ]
            )
        );
    }
}
