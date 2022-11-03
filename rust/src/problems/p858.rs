pub struct Solution;

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let k = gcd(p, q);
        let lcm = p * q / k;
        let m = lcm / p;
        let n = lcm / q;
        if n % 2 == 1 {
            i32::from(m % 2 == 1)
        } else {
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::mirror_reflection(2, 1));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::mirror_reflection(3, 1));
    }
}
