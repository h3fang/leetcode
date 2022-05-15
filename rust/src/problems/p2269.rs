pub struct Solution;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let mut n = num;
        let mut curr = 0;
        let mut i = 0;
        let mut result = 0;
        while n > 0 {
            let d = n % 10;
            if i < k {
                curr += d * 10i32.pow(i as u32);
                i += 1;
            } else {
                curr /= 10;
                curr += d * 10i32.pow(k as u32 - 1);
            }
            if i == k && curr != 0 && num % curr == 0 {
                result += 1;
            }
            n /= 10;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::divisor_substrings(240, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::divisor_substrings(30003, 3));
    }
}
