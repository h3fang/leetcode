pub struct Solution;

fn reverse(mut x: i32) -> i32 {
    let mut ans = 0;
    while x > 0 {
        ans = ans * 10 + x % 10;
        x /= 10;
    }
    ans
}

impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        (reverse(n) - n).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(27, Solution::mirror_distance(25));
    }

    #[test]
    fn case2() {
        assert_eq!(9, Solution::mirror_distance(10));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::mirror_distance(7));
    }
}
