pub struct Solution;

impl Solution {
    pub fn min_moves(target: i32, mut max_doubles: i32) -> i32 {
        let mut n = target;
        let mut k = 0;
        while max_doubles > 0 && n != 1 {
            if n % 2 == 1 {
                n -= 1;
            } else {
                n /= 2;
                max_doubles -= 1;
            }
            k += 1;
        }
        k + n - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::min_moves(5, 0));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::min_moves(19, 2));
    }
}
