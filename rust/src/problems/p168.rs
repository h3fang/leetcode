pub struct Solution;

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut result = vec![];
        while column_number > 0 {
            column_number -= 1;
            result.push(b'A' + (column_number % 26) as u8);
            column_number /= 26;
        }
        result.reverse();
        unsafe { String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("A", Solution::convert_to_title(1));
    }

    #[test]
    fn case2() {
        assert_eq!("AB", Solution::convert_to_title(28));
    }

    #[test]
    fn case3() {
        assert_eq!("ZY", Solution::convert_to_title(701));
    }
}
