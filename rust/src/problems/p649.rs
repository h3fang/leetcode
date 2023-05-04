pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut r = senate.as_bytes().iter().filter(|&&b| b == b'R').count();
        let mut d = senate.len() - r;
        let mut q = senate.as_bytes().iter().cloned().collect::<VecDeque<_>>();
        let (mut c_r, mut c_b) = (0, 0);
        while r > 0 && d > 0 {
            let c = q.pop_front().unwrap();
            match c {
                b'R' => {
                    if c_b > 0 {
                        c_b -= 1;
                        r -= 1;
                    } else {
                        c_r += 1;
                        q.push_back(c);
                    }
                }
                _ => {
                    if c_r > 0 {
                        c_r -= 1;
                        d -= 1;
                    } else {
                        c_b += 1;
                        q.push_back(c);
                    }
                }
            }
        }
        if r > 0 {
            "Radiant".to_string()
        } else {
            "Dire".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("Radiant", Solution::predict_party_victory("RD".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("Dire", Solution::predict_party_victory("RDD".to_string()));
    }
}
