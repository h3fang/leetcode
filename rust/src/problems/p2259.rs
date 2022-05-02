pub struct Solution;

impl Solution {
    pub fn remove_digit(mut number: String, digit: char) -> String {
        let n = number.as_bytes();
        let mut last = 0;
        for (i, c) in n.iter().enumerate() {
            if *c as char == digit {
                last = i;
                if i + 1 < n.len() && n[i + 1] > *c {
                    number.remove(i);
                    return number;
                }
            }
        }
        number.remove(last);
        number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("12", Solution::remove_digit("123".into(), '3'));
    }

    #[test]
    fn case2() {
        assert_eq!("231", Solution::remove_digit("1231".into(), '1'));
    }
}
