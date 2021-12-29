pub struct Solution;

#[derive(Default)]
struct Trie {
    next: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

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
    pub fn find_all_concatenated_words_in_a_dict(mut words: Vec<String>) -> Vec<String> {
        fn valid(trie: &Trie, w: &[u8]) -> bool {
            if w.is_empty() {
                return true;
            }
            let mut t = trie;
            for i in 0..w.len() {
                let j = (w[i] - b'a') as usize;
                if let Some(next) = &t.next[j] {
                    t = next.as_ref();
                } else {
                    return false;
                }
                if t.is_end && valid(trie, &w[i + 1..]) {
                    return true;
                }
            }
            false
        }
        words.sort_unstable_by_key(|w| w.len());
        let mut trie = Trie::new();
        let mut result = vec![];
        for word in words.into_iter().filter(|w| !w.is_empty()) {
            let w = word.as_bytes();
            if valid(&trie, w) {
                result.push(word);
            } else {
                trie.insert(w);
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
        let words = [
            "cat",
            "cats",
            "catsdogcats",
            "dog",
            "dogcatsdog",
            "hippopotamuses",
            "rat",
            "ratcatdogcat",
        ];
        let words = words.iter().map(|w| w.to_string()).collect();
        let expected = ["catsdogcats", "dogcatsdog", "ratcatdogcat"];
        let mut expected = expected.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::find_all_concatenated_words_in_a_dict(words);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
