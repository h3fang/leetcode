pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let n = n as u32;
        let mut seen = HashSet::with_capacity(10usize.pow(n.div_ceil(2)));
        let start = 10i32.pow((n - 1) / 2);
        for x in start..(start * 10) {
            let mut bytes = x.to_string().into_bytes();
            bytes.reserve(n as usize);
            for i in n.div_ceil(2)..n {
                bytes.push(bytes[(n - i - 1) as usize]);
            }
            let s = unsafe { std::str::from_utf8_unchecked(&bytes) };
            let x: i64 = s.parse().unwrap();
            if x % k as i64 == 0 {
                bytes.sort_unstable();
                seen.insert(bytes);
            }
        }

        let mut frac = [1; 11];
        for i in 1..11 {
            frac[i] = frac[i - 1] * i as i64;
        }
        let n = n as i64;
        let mut ans = 0;
        for bytes in seen {
            let mut freq = [0; 10];
            for b in bytes {
                freq[(b - b'0') as usize] += 1;
            }
            let denom: i64 = freq.iter().map(|&f| frac[f as usize]).product();
            ans += (n - freq[0]) * frac[n as usize - 1] / denom;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(27, Solution::count_good_integers(3, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::count_good_integers(1, 4));
    }

    #[test]
    fn case3() {
        assert_eq!(2468, Solution::count_good_integers(5, 6));
    }
}
