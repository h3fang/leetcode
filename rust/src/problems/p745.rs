pub struct WordFilter {
    trie: Trie,
}

#[derive(Default)]
struct Trie {
    next: [Option<Box<Trie>>; 27],
    idx: i32,
}

impl Trie {
    fn insert(&mut self, s: u8) -> &mut Self {
        let i = (s - b'a') as usize;
        self.next[i].get_or_insert_with(Default::default)
    }
}

impl WordFilter {
    pub fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::default();
        for (i, w) in words.iter().enumerate() {
            let w = w.as_bytes();
            for j in 0..w.len() {
                let mut t = &mut trie;
                for &c in &w[j..] {
                    t = t.insert(c);
                }
                t = t.insert(b'{');
                t.idx = t.idx.max(i as i32 + 1);
                for s in w {
                    t = t.insert(*s);
                    t.idx = t.idx.max(i as i32 + 1);
                }
            }
        }
        Self { trie }
    }

    pub fn f(&self, prefix: String, suffix: String) -> i32 {
        let mut t = &self.trie;
        for c in suffix
            .as_bytes()
            .iter()
            .chain(b"{")
            .chain(prefix.as_bytes())
        {
            let i = (c - b'a') as usize;
            if let Some(next) = &t.next[i] {
                t = next;
            } else {
                return -1;
            }
        }
        t.idx - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let wf = WordFilter::new(vec!["apple".to_string()]);
        assert_eq!(0, wf.f("a".into(), "e".into()));
    }
}
