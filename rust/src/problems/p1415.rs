pub struct Solution;

impl Solution {
    pub fn get_happy_string(n: i32, mut k: i32) -> String {
        let mut m = 1 << (n - 1);
        if k > 3 * m {
            return String::new();
        }
        k -= 1;
        let mut result = Vec::with_capacity(n as usize);
        if k < m {
            result.push(b'a');
        } else if k < 2 * m {
            result.push(b'b');
        } else {
            result.push(b'c');
        };
        k %= m;
        m /= 2;
        for _ in 2..=n {
            let j = if k < m { 1 } else { 2 };
            let last = *result.last().unwrap();
            if last == b'b' {
                if j == 1 {
                    result.push(b'a');
                } else {
                    result.push(b'c');
                }
            } else {
                result.push(b'a' + (last - b'a' + j) % 3);
            }
            k %= m;
            m /= 2;
        }
        String::from_utf8(result).unwrap()
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
