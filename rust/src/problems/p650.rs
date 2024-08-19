pub struct Solution;

impl Solution {
    pub fn min_steps(mut n: i32) -> i32 {
        let (mut result, mut i) = (0, 2);
        while i * i <= n {
            while n % i == 0 {
                result += i;
                n /= i;
            }
            i += 1;
        }
        if n > 1 {
            result += n;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::min_steps(3));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_steps(1));
    }
}
