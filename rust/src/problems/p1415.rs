pub struct Solution;

impl Solution {
    pub fn get_happy_string(n: i32, mut k: i32) -> String {
        if k > 3 << (n - 1) {
            return String::new();
        }
        k -= 1;
        let mut result = Vec::with_capacity(n as usize);
        let mut last = b'a' + (k >> (n - 1)) as u8;
        result.push(last);
        for i in 1..n {
            let b = b'a' + ((k >> (n - 1 - i)) & 1) as u8;
            last = b + u8::from(b >= last);
            result.push(last);
        }
        unsafe { String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("c", Solution::get_happy_string(1, 3));
    }

    #[test]
    fn case2() {
        assert_eq!("", Solution::get_happy_string(1, 4));
    }

    #[test]
    fn case3() {
        assert_eq!("cab", Solution::get_happy_string(3, 9));
    }

    #[test]
    fn case4() {
        assert_eq!("abacbabacb", Solution::get_happy_string(10, 100));
    }
}
