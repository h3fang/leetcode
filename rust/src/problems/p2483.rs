pub struct Solution;

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut result = 0;
        let mut no = 0;
        for (i, &b) in customers.as_bytes().iter().enumerate() {
            if b == b'N' {
                no += 1;
            } else {
                no -= 1;
                if no < 0 {
                    no = 0;
                    result = i + 1;
                }
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::best_closing_time("YYNY".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::best_closing_time("NNNNN".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::best_closing_time("YYYY".to_string()));
    }
}
