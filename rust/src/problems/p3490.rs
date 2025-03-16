pub struct Solution;

use std::collections::HashMap;

fn dfs(
    digits: &[i32],
    i: usize,
    limited: bool,
    sum: i32,
    product: i32,
    cache: &mut HashMap<(usize, bool, i32, i32), i32>,
) -> i32 {
    if i == digits.len() {
        return i32::from(sum > 0 && product % sum == 0);
    }
    let key = (i, limited, sum, product);
    if let Some(&r) = cache.get(&key) {
        return r;
    }
    let mut ans = 0;
    let max = if limited { digits[i] } else { 9 };
    for d in 0..=max {
        let sum1 = sum + d;
        let product1 = if sum1 > 0 { product * d } else { product };
        ans += dfs(digits, i + 1, limited && d == max, sum1, product1, cache);
    }
    cache.insert(key, ans);
    ans
}

fn count(mut x: i32) -> i32 {
    let mut digits = Vec::with_capacity(10);
    while x > 0 {
        digits.push(x % 10);
        x /= 10;
    }
    digits.reverse();
    let mut cache = HashMap::new();
    dfs(&digits, 0, true, 0, 1, &mut cache)
}

impl Solution {
    pub fn beautiful_numbers(l: i32, r: i32) -> i32 {
        count(r) - count(l - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::beautiful_numbers(10, 20))
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::beautiful_numbers(1, 15))
    }

    #[test]
    fn case3() {
        assert_eq!(5, Solution::beautiful_numbers(571, 581))
    }

    #[test]
    fn case4() {
        assert_eq!(15, Solution::beautiful_numbers(20, 100))
    }
}
