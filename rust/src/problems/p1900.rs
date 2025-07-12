pub struct Solution;

fn earliest(mut n: i32, f: i32, s: i32) -> i32 {
    let mut r = 1;

    if f + s <= (n + 1) / 2 {
        let k = 31 - ((n - 1) / (f + s - 1)).leading_zeros();
        n = ((n - 1) >> k) + 1;
        r += k as i32;
        if s - f > 1 {
            return r + 1;
        }
    }

    if s - f == 1 || (s > (n + 1) / 2 && (s - f) == 2) {
        return r + 1 + ((n - 1) / 2).trailing_zeros() as i32;
    }

    if s > (n + 1) / 2 && f % 2 == 0 && f + s == n {
        r += 1;
    }

    r + 1
}

impl Solution {
    pub fn earliest_and_latest(n: i32, mut f: i32, mut s: i32) -> Vec<i32> {
        if f + s == n + 1 {
            return vec![1, 1];
        }
        if f + s > n + 1 {
            (f, s) = (n + 1 - s, n + 1 - f);
        }
        let e = earliest(n, f, s);
        let l = (32 - (n - 1).leading_zeros() as i32).min(n + 1 - s);
        vec![e, l]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![3, 4], Solution::earliest_and_latest(11, 2, 4));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![1, 1], Solution::earliest_and_latest(5, 1, 5));
    }
}
