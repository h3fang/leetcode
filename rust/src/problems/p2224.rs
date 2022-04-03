pub struct Solution;

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        fn parse_time(t: &str) -> i32 {
            let p = t.split_once(':').unwrap();
            p.0.parse::<i32>().unwrap() * 60 + p.1.parse::<i32>().unwrap()
        }
        let mut delta = parse_time(&correct) - parse_time(&current);
        let mut result = 0;
        result += delta / 60;
        delta %= 60;
        result += delta / 15;
        delta %= 15;
        result += delta / 5;
        delta %= 5;
        result + delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::convert_time("02:30".to_string(), "04:35".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::convert_time("11:00".to_string(), "11:01".to_string())
        );
    }
}
