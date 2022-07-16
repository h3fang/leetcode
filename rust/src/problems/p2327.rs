pub struct Solution;

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        const MOD: i64 = 10_0000_0007;
        let mut f = vec![0i64; n as usize];
        let mut aware = 0;
        f[0] = 1;
        for i in 0..f.len() {
            let v = f[i];
            if i as i32 + delay >= n {
                aware += v;
            }
            let a = i + delay as usize;
            let b = (i + forget as usize).min(n as usize);
            for j in f.iter_mut().take(b).skip(a) {
                *j = (*j + v) % MOD;
            }
        }

        ((f.last().unwrap() + aware) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::people_aware_of_secret(6, 2, 4));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::people_aware_of_secret(4, 1, 3));
    }
}
