pub struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut max: Option<char> = None;
        let mut count = 0;
        let mut last = ' ';
        for c in num.chars() {
            if c == last {
                count += 1;
                if count == 3 {
                    match max {
                        Some(m) => max = Some(m.max(c)),
                        None => max = Some(c),
                    }
                }
            } else {
                count = 1;
            }
            last = c;
        }
        match max {
            Some(m) => std::iter::repeat_n(m, 3).collect(),
            None => "".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("777", Solution::largest_good_integer("6777133339".into()));
    }

    #[test]
    fn case2() {
        assert_eq!("000", Solution::largest_good_integer("2300019".into()));
    }

    #[test]
    fn case3() {
        assert_eq!("", Solution::largest_good_integer("42352338".into()));
    }
}
