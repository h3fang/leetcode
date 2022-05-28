pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn total_strength(strength: Vec<i32>) -> i32 {
        let n = strength.len();

        let mut st = vec![];
        let mut left = vec![-1; n];
        let mut right = vec![n as i32; n];
        for (i, &s) in strength.iter().enumerate() {
            while let Some(&top) = st.last() {
                if strength[top] <= s {
                    break;
                }
                right[top] = i as i32;
                st.pop();
            }
            if let Some(&top) = st.last() {
                left[i] = top as i32;
            }
            st.push(i);
        }

        let mut psum = strength.iter().map(|s| *s as i64).collect::<Vec<_>>();
        for i in 1..n {
            psum[i] = (psum[i] + psum[i - 1]) % MOD;
        }

        let mut ppsum = psum.clone();
        for i in 1..n {
            ppsum[i] = (ppsum[i] + ppsum[i - 1]) % MOD;
        }

        let f = |l: i32, r: i32| {
            if r < 0 {
                0
            } else if l < 0 {
                ppsum[r as usize]
            } else {
                (ppsum[r as usize] - ppsum[l as usize] + MOD) % MOD
            }
        };

        let mut result = 0;
        for i in 0..n {
            let l = left[i] + 1;
            let r = right[i] - 1;
            let p1 = (f(i as i32 - 1, r) * (i as i64 - l as i64 + 1)) % MOD;
            let p2 = (f(l - 2, i as i32 - 1) * (r as i64 - i as i64 + 1)) % MOD;
            let contrib = (strength[i] as i64 * ((p1 - p2 + MOD) % MOD)) % MOD;
            result = (result + contrib) % MOD;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(44, Solution::total_strength(vec![1, 3, 1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(213, Solution::total_strength(vec![5, 4, 6]));
    }
}
