pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        let k = (r - l + 1) as usize;
        let mut f = vec![1; k];

        for i in 1..n {
            if i % 2 == 0 {
                let mut pre = 0;
                for e in f.iter_mut() {
                    (*e, pre) = (pre, (pre + *e) % MOD);
                }
            } else {
                let mut suf = 0;
                for e in f.iter_mut().rev() {
                    (*e, suf) = (suf, (suf + *e) % MOD);
                }
            }
        }

        ((f.iter().sum::<i64>() * 2) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::zig_zag_arrays(3, 4, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::zig_zag_arrays(3, 1, 3));
    }
}
