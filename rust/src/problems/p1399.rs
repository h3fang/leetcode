pub struct Solution;

use std::collections::HashMap;

fn dfs(
    digits: &[i8],
    i: usize,
    sum: i8,
    limit_high: bool,
    cache: &mut HashMap<(usize, i8), i32>,
) -> i32 {
    if i == digits.len() {
        return i32::from(sum == 0);
    }
    if !limit_high && let Some(&r) = cache.get(&(i, sum)) {
        return r;
    }
    let hi = if limit_high { digits[i] } else { 9 };
    let mut ans = 0;
    for d in 0..=hi.min(sum) {
        ans += dfs(digits, i + 1, sum - d, limit_high && d == hi, cache);
    }
    if !limit_high {
        cache.insert((i, sum), ans);
    }
    ans
}

impl Solution {
    pub fn count_largest_group(mut n: i32) -> i32 {
        let mut digits = Vec::with_capacity(4);
        while n > 0 {
            digits.push((n % 10) as i8);
            n /= 10;
        }
        digits.reverse();
        let m = digits.len();
        let mut cache = HashMap::with_capacity(1024);
        let (mut max, mut ans) = (0, 0);
        for sum in 1..=m as i8 * 9 {
            let c = dfs(&digits, 0, sum, true, &mut cache);
            match c.cmp(&max) {
                std::cmp::Ordering::Equal => ans += 1,
                std::cmp::Ordering::Greater => {
                    max = c;
                    ans = 1;
                }
                _ => {}
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::count_largest_group(13));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::count_largest_group(2));
    }

    #[test]
    fn case3() {
        assert_eq!(6, Solution::count_largest_group(15));
    }

    #[test]
    fn case4() {
        assert_eq!(5, Solution::count_largest_group(24));
    }
}
