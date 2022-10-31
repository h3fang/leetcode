pub struct Solution;

impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        if n < 4 {
            return 1;
        }
        let mut s = String::with_capacity(n as usize);
        s.push_str("122");
        let mut result = 1;
        let mut i = 2;
        let mut j = 3;
        while j < n as usize {
            let count = (s.as_bytes()[i] - b'0') as i32;
            let num = match s.as_bytes()[j - 1] {
                b'1' => '2',
                _ => '1',
            };
            for _ in 0..count {
                if j == n as usize {
                    break;
                }
                s.push(num);
                if num == '1' {
                    result += 1;
                }
                j += 1;
            }
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::magical_string(6));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::magical_string(1));
    }
}
