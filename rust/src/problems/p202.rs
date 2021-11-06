pub struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        fn next(mut n: i32) -> i32 {
            let mut next = 0;
            while n > 0 {
                let d = n % 10;
                next += d * d;
                n /= 10;
            }
            next
        }

        let mut slow = n;
        let mut fast = next(n);
        while slow != fast {
            slow = next(slow);
            fast = next(next(fast));
        }
        fast == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::is_happy(19));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::is_happy(2));
    }
}
