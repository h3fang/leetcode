pub struct Solution;

impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        if num * 9 < sum {
            return String::new();
        }

        let num = num as usize;

        let mut ans = Vec::with_capacity(num);
        ans.extend(std::iter::repeat_n(b'9', sum as usize / 9));
        ans.push(b'0' + (sum % 9) as u8);
        ans.resize(num, b'0');

        unsafe { String::from_utf8_unchecked(ans) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("30", Solution::max_sum_of_squares(2, 3));
    }

    #[test]
    fn case2() {
        assert_eq!("98", Solution::max_sum_of_squares(2, 17));
    }

    #[test]
    fn case3() {
        assert_eq!("", Solution::max_sum_of_squares(1, 10));
    }
}
