pub struct Solution;

impl Solution {
    pub fn reordered_power_of2_backtrack(mut n: i32) -> bool {
        let mut digits = Vec::new();
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        digits.sort_unstable();

        fn is_power_of_two(n: i64) -> bool {
            n & (n - 1) == 0
        }

        fn bt(digits: &[i32], i: usize, curr: i64, used: &mut [bool]) -> bool {
            if i == digits.len() {
                is_power_of_two(curr)
            } else {
                let mut prev = -1;
                for (j, &d) in digits.iter().enumerate() {
                    if used[j] || d == prev || (curr == 0 && d == 0) {
                        continue;
                    }

                    let next = curr * 10 + d as i64;
                    used[j] = true;
                    if bt(digits, i + 1, next, used) {
                        return true;
                    }
                    used[j] = false;

                    prev = d;
                }
                false
            }
        }

        let mut used = vec![false; digits.len()];
        bt(&digits, 0, 0, &mut used)
    }

    pub fn reordered_power_of2(n: i32) -> bool {
        fn digits(mut n: i32) -> Vec<i32> {
            let mut r = Vec::new();
            while n > 0 {
                r.push(n % 10);
                n /= 10;
            }
            r.sort_unstable();
            r
        }

        let d = digits(n);
        (0..31).map(|i| digits(1 << i)).any(|x| x == d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::reordered_power_of2_backtrack(1));
        assert_eq!(true, Solution::reordered_power_of2(1));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::reordered_power_of2_backtrack(10));
        assert_eq!(false, Solution::reordered_power_of2(10));
    }

    #[test]
    fn case3() {
        assert_eq!(true, Solution::reordered_power_of2_backtrack(16));
        assert_eq!(true, Solution::reordered_power_of2(16));
    }

    #[test]
    fn case4() {
        assert_eq!(false, Solution::reordered_power_of2_backtrack(24));
        assert_eq!(false, Solution::reordered_power_of2(24));
    }

    #[test]
    fn case5() {
        assert_eq!(true, Solution::reordered_power_of2_backtrack(46));
        assert_eq!(true, Solution::reordered_power_of2(46));
    }
}
