pub struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n = n as i64;
        let mut k = 1;
        let mut lb = 1i64;
        while n > (lb * 10 - lb) * k {
            n -= (lb * 10 - lb) * k;
            lb *= 10;
            k += 1;
        }
        let i = (n - 1) / k;
        let d = (n - 1) % k;
        n = lb + i;
        for _ in 0..(k - d - 1) {
            n /= 10;
        }
        (n % 10) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::find_nth_digit(5));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::find_nth_digit(11));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::find_nth_digit(10));
    }

    #[test]
    fn case4() {
        assert_eq!(1, Solution::find_nth_digit(1000000000));
    }
}
