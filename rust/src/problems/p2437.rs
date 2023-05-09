pub struct Solution;

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let t = time.as_bytes();
        let mut result = match (t[0], t[1]) {
            (b'?', b'?') => 24,
            (x, b'?') if x < b'2' => 10,
            (_, b'?') => 4,
            (b'?', x) if x < b'4' => 3,
            (b'?', _) => 2,
            _ => 1,
        };
        if t[3] == b'?' {
            result *= 6;
        };
        if t[4] == b'?' {
            result *= 10;
        };
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::count_time("?5:00".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(100, Solution::count_time("0?:0?".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(1440, Solution::count_time("??:??".to_string()));
    }
}
