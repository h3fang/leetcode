pub struct Solution;

#[derive(Default, Clone)]
struct Trie {
    next: [Option<Box<Trie>>; 26],
    score: i32,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, w: &[u8]) {
        let mut t = self;
        for b in w {
            t = t.next[(b - b'a') as usize].get_or_insert_with(|| Box::new(Trie::default()));
            t.score += 1;
        }
    }
}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::new();
        for w in &words {
            trie.insert(w.as_bytes());
        }
        words
            .iter()
            .map(|w| {
                let mut c = 0;
                let mut t = &trie;
                for b in w.as_bytes() {
                    if let Some(n) = &t.next[(b - b'a') as usize] {
                        c += n.score;
                        t = n;
                    } else {
                        break;
                    }
                }
                c
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["abc", "ab", "bc", "b"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(vec![5, 4, 3, 2], Solution::sum_prefix_scores(words));
    }

    #[test]
    fn case2() {
        let words = ["abcd"].iter().map(|w| w.to_string()).collect();
        assert_eq!(vec![4], Solution::sum_prefix_scores(words));
    }
}
