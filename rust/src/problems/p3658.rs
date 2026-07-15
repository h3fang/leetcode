pub struct Solution;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 {
        (a, b) = (b % a, a);
    }
    b
}

impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        let odd = (1 + 1 + 2 * (n - 1)) * n / 2;
        let even = odd + n;
        gcd(odd, even)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::gcd_of_odd_even_sums(4));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::gcd_of_odd_even_sums(5));
    }
}
