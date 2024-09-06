pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut curr = "1".to_string();
        for _ in 1..n {
            let mut next = String::new();
            let mut last = curr.as_bytes()[0];
            let mut count = 1;
            for &b in curr.as_bytes().iter().chain(b" ").skip(1) {
                if b != last {
                    next += &count.to_string();
                    next.push(last as char);
                    count = 1;
                } else {
                    count += 1;
                }
                last = b;
            }
            curr = next;
        }
        curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("1", Solution::count_and_say(1));
    }

    #[test]
    fn case2() {
        assert_eq!("1211", Solution::count_and_say(4));
    }
}
