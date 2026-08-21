pub struct Solution;

fn check(lcms: &[i64], m: i64, k: i64) -> bool {
    let mut cnt = 0;
    for (i, v) in lcms.iter().enumerate().skip(1) {
        if i.count_ones() % 2 == 1 {
            cnt += m / v;
        } else {
            cnt -= m / v;
        }
    }
    cnt >= k
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a > 0 {
        (a, b) = (b % a, a);
    }
    b
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

impl Solution {
    pub fn find_kth_smallest(mut coins: Vec<i32>, k: i32) -> i64 {
        coins.sort_unstable();

        let mut n = 0;
        for i in 0..coins.len() {
            let x = coins[i];
            if coins[..i].iter().all(|&y| x % y != 0) {
                coins[n] = x;
                n += 1;
            }
        }

        coins.truncate(n);

        let mut lcms = vec![1; 1 << coins.len()];
        for (i, &x) in coins.iter().enumerate() {
            let bits = 1 << i;
            for m in 0..bits {
                lcms[bits | m] = lcm(lcms[m], i64::from(x));
            }
        }

        let k = i64::from(k);
        let (mut l, mut r) = (k - 1, coins[0] as i64 * k);
        while l + 1 < r {
            let m = l.midpoint(r);
            if check(&lcms, m, k) {
                r = m;
            } else {
                l = m;
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9, Solution::find_kth_smallest(vec![3, 6, 9], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(12, Solution::find_kth_smallest(vec![5, 2], 7));
    }
}
