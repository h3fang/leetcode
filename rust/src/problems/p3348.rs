pub struct Solution;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a > 0 {
        (a, b) = (b % a, a);
    }
    b
}

impl Solution {
    pub fn smallest_number(mut num: String, t: i64) -> String {
        {
            let mut t = t;
            for p in [2, 3, 5, 7] {
                while t % p == 0 {
                    t /= p;
                }
            }

            if t > 1 {
                return String::from("-1");
            }
        }

        let n = num.len();
        let mut left_t = vec![0; n + 1];
        left_t[0] = t;

        let mut i0 = n - 1;
        for (i, &b) in num.as_bytes().iter().enumerate() {
            if b == b'0' {
                i0 = i;
                break;
            }

            left_t[i + 1] = left_t[i] / gcd(left_t[i], i64::from(b - b'0'));
        }

        if left_t[n] == 1 {
            return num;
        }

        let s = unsafe { num.as_bytes_mut() };

        for i in (0..=i0).rev() {
            s[i] += 1;
            while s[i] <= b'9' {
                let mut t1 = left_t[i] / gcd(left_t[i], i64::from(s[i] - b'0'));
                let mut k = 9;
                for j in (i + 1..n).rev() {
                    while t1 % k != 0 {
                        k -= 1;
                    }

                    t1 /= k;
                    s[j] = k as u8 + b'0';
                }
                if t1 == 1 {
                    return num;
                }
                s[i] += 1;
            }
        }

        let mut ans = Vec::with_capacity(n + 1);
        let mut t = t;

        for d in (2..10).rev() {
            while t % d == 0 {
                ans.push(d as u8 + b'0');
                t /= d;
            }
        }
        while ans.len() < n + 1 {
            ans.push(b'1');
        }
        ans.reverse();

        unsafe { String::from_utf8_unchecked(ans) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "1488".to_string(),
            Solution::smallest_number("1234".to_string(), 256)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "12355".to_string(),
            Solution::smallest_number("12355".to_string(), 50)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "-1".to_string(),
            Solution::smallest_number("11111".to_string(), 26)
        );
    }
}
