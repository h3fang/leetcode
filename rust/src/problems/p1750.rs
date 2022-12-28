pub struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            match s[i].cmp(&s[j]) {
                std::cmp::Ordering::Equal => {
                    let c = s[i];
                    while i <= j && s[i] == c {
                        i += 1;
                    }
                    while i <= j && s[j] == c {
                        j -= 1;
                    }
                }
                _ => break,
            }
        }
        (j + 1 - i) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::minimum_length("ca".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::minimum_length("cabaabac".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::minimum_length("aabccabba".to_string()));
    }
}
