pub struct Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut result = 0;
        let mut inside = false;
        for &b in s.as_bytes() {
            match b {
                b'|' => {
                    inside = !inside;
                }
                b'*' => {
                    if !inside {
                        result += 1;
                    }
                }
                _ => {}
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::count_asterisks("l|*e*et|c**o|*de|".into()));
    }
}
