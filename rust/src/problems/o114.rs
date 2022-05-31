pub struct Solution;

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut g: HashMap<u8, Vec<u8>> = HashMap::new();
        let mut indegrees = HashMap::new();
        for w in &words {
            for &b in w.as_bytes() {
                g.entry(b).or_default();
            }
        }
        for w in words.windows(2) {
            let w0 = w[0].as_bytes();
            let w1 = w[1].as_bytes();
            let mut equal = true;
            for (&a, &b) in w0.iter().zip(w1) {
                if a != b {
                    g.entry(a).or_default().push(b);
                    *indegrees.entry(b).or_insert(0) += 1;
                    equal = false;
                    break;
                }
            }
            if equal && w0.len() > w1.len() {
                return "".into();
            }
        }
        let mut q = VecDeque::new();
        for &b in g.keys() {
            if *indegrees.get(&b).unwrap_or(&0) == 0 {
                q.push_back(b);
            }
        }
        let mut order = String::new();
        while let Some(b) = q.pop_front() {
            order.push(b as char);
            if let Some(neighbors) = g.get(&b) {
                for &next in neighbors {
                    let e = indegrees.entry(next).or_default();
                    *e -= 1;
                    if *e == 0 {
                        q.push_back(next);
                    }
                }
            }
        }
        if order.len() == g.len() {
            order
        } else {
            "".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_valid(order: &str, words: Vec<String>) {
        let mut o = [usize::MAX; 26];
        for (i, b) in order.as_bytes().iter().enumerate() {
            o[(b - b'a') as usize] = i;
        }

        let cmp_alien_word = |a: &String, b: &String| {
            for (b1, b2) in a.as_bytes().iter().zip(b.as_bytes()) {
                if b1 == b2 {
                    continue;
                }
                let i1 = o[(b1 - b'a') as usize];
                let i2 = o[(b2 - b'a') as usize];
                return i1.cmp(&i2);
            }
            a.len().cmp(&b.len())
        };
        for w in words.windows(2) {
            assert_eq!(std::cmp::Ordering::Less, cmp_alien_word(&w[0], &w[1]));
        }
    }

    #[test]
    fn case1() {
        let words = ["wrt", "wrf", "er", "ett", "rftt"];
        let words = words.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        assert_valid(&Solution::alien_order(words.clone()), words);
    }

    #[test]
    fn case2() {
        let words = ["z", "x", "z"];
        let words = words.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        assert!(Solution::alien_order(words.clone()).is_empty());
    }
}
