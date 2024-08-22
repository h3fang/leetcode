pub struct Solution;

fn accumulate_bit(i: u32, n: i64) -> i64 {
    let period = 1 << i;
    let result = period / 2 * (n / period);
    if n % period >= period / 2 {
        result + n % period - (period / 2 - 1)
    } else {
        result
    }
}

fn accumulate(n: i64, x: i32) -> i64 {
    let mut result = 0;
    for i in (x as u32..=(64 - n.leading_zeros())).step_by(x as usize) {
        result += accumulate_bit(i, n);
    }
    result
}

impl Solution {
    pub fn find_maximum_number(k: i64, x: i32) -> i64 {
        let (mut l, mut r) = (1, (k + 1) << x);
        while l < r {
            let m = l + (r - l + 1) / 2;
            if accumulate(m, x) > k {
                r = m - 1;
            } else {
                l = m;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::find_maximum_number(9, 1));
    }

    #[test]
    fn case2() {
        assert_eq!(9, Solution::find_maximum_number(7, 2));
    }
}
