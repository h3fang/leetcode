use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut pos = [0u8; 26];
        for (i, &b) in order.as_bytes().iter().enumerate() {
            let j = (b - b'a') as usize;
            pos[j] = i as u8;
        }
        words.windows(2).all(|w| {
            let a = w[0].as_bytes();
            let b = w[1].as_bytes();
            for (a, b) in a.iter().zip(b) {
                let i = (a - b'a') as usize;
                let j = (b - b'a') as usize;
                match pos[i].cmp(&pos[j]) {
                    Ordering::Less => return true,
                    Ordering::Equal => {}
                    Ordering::Greater => return false,
                }
            }
            if a.len() > b.len() {
                return false;
            }
            true
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = vec!["hello".to_string(), "leetcode".to_string()];
        let order = "hlabcdefgijkmnopqrstuvwxyz".to_string();
        assert_eq!(true, Solution::is_alien_sorted(words, order));
    }
}
