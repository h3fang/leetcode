pub struct Solution;

#[derive(Default)]
struct Trie {
    next: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn insert(&mut self, w: &[u8]) {
        let mut t = self;
        for b in w {
            let i = (b - b'a') as usize;
            t = t.next[i].get_or_insert_with(Default::default);
        }
        t.is_end = true;
    }
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::default();
        for d in dictionary {
            trie.insert(d.as_bytes());
        }
        let mut result = String::with_capacity(sentence.len());
        for s in sentence.split_ascii_whitespace() {
            if !result.is_empty() {
                result.push(' ');
            }
            let mut t = &trie;
            let mut found = false;
            for (i, b) in s.as_bytes().iter().enumerate() {
                let j = (b - b'a') as usize;
                if let Some(n) = &t.next[j] {
                    t = n;
                    if t.is_end {
                        result.push_str(&s[..=i]);
                        found = true;
                        break;
                    }
                } else {
                    break;
                }
            }
            if !found {
                result.push_str(s);
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
        let dictionary = ["cat", "bat", "rat"];
        let dictionary = dictionary.iter().map(|s| s.to_string()).collect();
        let sentence = "the cattle was rattled by the battery".to_string();
        assert_eq!(
            "the cat was rat by the bat",
            Solution::replace_words(dictionary, sentence)
        );
    }
}
