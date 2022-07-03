pub struct Solution;

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut n = n as i64;
        let mut digits: Vec<i64> = Vec::with_capacity(16);
        while n > 0 {
            let d = n % 10;
            n /= 10;
            if let Some(&e) = digits.last() {
                if e > d {
                    for (i, &e) in digits.iter().enumerate() {
                        if e > d {
                            n = n * 10 + e;
                            digits[i] = d;
                            break;
                        }
                    }
                    for d in digits {
                        n = n * 10 + d;
                    }
                    if n > i32::MAX as i64 {
                        return -1;
                    } else {
                        return n as i32;
                    }
                }
            }
            digits.push(d);
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(21, Solution::next_greater_element(12));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::next_greater_element(21));
    }
}
