pub struct Solution;

impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let (mut m1, mut m2) = (0, 0);
        while n > 0 {
            let d = n % 10;
            if d > m1 {
                m2 = m1;
                m1 = d;
            } else if d > m2 {
                m2 = d;
            }
            n /= 10;
        }
        m1 * m2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::max_product(31));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::max_product(22));
    }

    #[test]
    fn case3() {
        assert_eq!(8, Solution::max_product(124));
    }
}
