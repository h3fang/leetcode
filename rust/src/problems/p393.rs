pub struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        fn is_valid_utf8_char(bytes: &[i32]) -> bool {
            if bytes.len() == 1 {
                false
            } else {
                for b in bytes.iter().skip(1) {
                    let first_two = *b & 0b10000000;
                    if first_two != 0b10000000 {
                        return false;
                    }
                }
                true
            }
        }

        let mut i = 0;
        while i < data.len() {
            let first = data[i] as u8;
            let leading_ones = first.leading_ones() as usize;
            if leading_ones > 4 || i + leading_ones > data.len() {
                return false;
            } else if leading_ones == 0 {
                i += 1;
            } else if is_valid_utf8_char(&data[i..i + leading_ones]) {
                i += leading_ones;
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::valid_utf8(vec![197, 130, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::valid_utf8(vec![235, 140, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(true, Solution::valid_utf8(vec![230, 136, 145]));
    }
}
