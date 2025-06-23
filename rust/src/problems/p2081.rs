pub struct Solution;

use std::sync::OnceLock;

static F: OnceLock<[Vec<i64>; 10]> = OnceLock::new();

fn is_k_palindrome(mut x: i64, k: i64) -> bool {
    if x % k == 0 {
        return false;
    }
    let mut rev = 0;
    while rev < x / k {
        rev = rev * k + x % k;
        x /= k;
    }
    rev == x || rev == x / k
}

fn find_palindromes(x: i64, f: &mut [Vec<i64>]) -> bool {
    let mut done = true;
    for (k, f) in f.iter_mut().enumerate().skip(2) {
        if f.len() < 30 && is_k_palindrome(x, k as i64) {
            f.push(x);
        }
        if f.len() < 30 {
            done = false;
        }
    }
    if done {
        for f in f.iter_mut().skip(2) {
            for i in 1..f.len() {
                f[i] += f[i - 1];
            }
        }
    }
    done
}

fn init() -> [Vec<i64>; 10] {
    let mut f: [Vec<i64>; 10] = [const { Vec::new() }; 10];
    f.iter_mut().skip(2).for_each(|f| f.reserve(30));
    let mut base = 1;
    loop {
        for round in 0..2 {
            for i in base..base * 10 {
                let mut x = i;
                let mut i = if round == 0 { i / 10 } else { i };
                while i > 0 {
                    x = x * 10 + i % 10;
                    i /= 10;
                }
                if find_palindromes(x, &mut f) {
                    return f;
                }
            }
        }
        base *= 10;
    }
}

impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        let f = F.get_or_init(init);
        f[k as usize][n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(25, Solution::k_mirror(2, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(499, Solution::k_mirror(3, 7));
    }

    #[test]
    fn case3() {
        assert_eq!(20379000, Solution::k_mirror(7, 17));
    }
}
