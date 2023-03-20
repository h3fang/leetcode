pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let mut visited = HashSet::new();
        let mut min = s.to_string();
        let mut queue = VecDeque::new();
        queue.push_back(s.to_string());
        visited.insert(s.to_string());
        let i = s.len() - b as usize;
        while let Some(mut curr) = queue.pop_front() {
            if curr < min {
                min = curr.to_string();
            }

            let step = format!("{}{}", &curr[i..], &curr[..i]);
            if visited.insert(step.to_string()) {
                queue.push_back(step);
            }

            unsafe { curr.as_bytes_mut() }
                .iter_mut()
                .skip(1)
                .step_by(2)
                .for_each(|c| *c = (*c - b'0' + a as u8) % 10 + b'0');
            if visited.insert(curr.to_string()) {
                queue.push_back(curr);
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "2050",
            Solution::find_lex_smallest_string("5525".to_string(), 9, 2)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "24",
            Solution::find_lex_smallest_string("74".to_string(), 5, 1)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "0011",
            Solution::find_lex_smallest_string("0011".to_string(), 4, 2)
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            "00553311",
            Solution::find_lex_smallest_string("43987654".to_string(), 7, 3)
        );
    }
}
