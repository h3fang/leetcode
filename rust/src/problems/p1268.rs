pub struct Solution;

#[derive(Default)]
struct Trie {
    next: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn insert(&mut self, w: &[u8]) {
        let mut t = self;
        for c in w {
            let i = (c - b'a') as usize;
            t = t.next[i].get_or_insert_with(Default::default);
        }
        t.is_end = true;
    }

    fn words(&self, prefix: &str, result: &mut Vec<String>) {
        fn dfs(t: &Trie, s: &mut String, prefix: &str, result: &mut Vec<String>) {
            if t.is_end && result.len() < 3 {
                result.push(prefix.to_string() + s);
            }
            if result.len() == 3 {
                return;
            }
            for i in 0..26 {
                if let Some(t) = &t.next[i] {
                    let c = b'a' + i as u8;
                    s.push(c as char);
                    dfs(t, s, prefix, result);
                    s.pop();
                }
            }
        }
        dfs(self, &mut String::with_capacity(3000), prefix, result);
    }
}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut trie = Trie::default();
        for p in products {
            trie.insert(p.as_bytes());
        }
        let mut result = vec![];
        let mut t = &trie;
        for (i, c) in search_word.as_bytes().iter().enumerate() {
            let j = (c - b'a') as usize;
            if let Some(next) = &t.next[j] {
                let mut words = Vec::with_capacity(3);
                next.words(&search_word[..=i], &mut words);
                t = next;
                result.push(words);
            } else {
                for _ in i..search_word.as_bytes().len() {
                    result.push(vec![]);
                }
                break;
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
        let products = ["mobile", "mouse", "moneypot", "monitor", "mousepad"];
        let products = products.iter().map(|p| p.to_string()).collect();
        let expected = [
            vec!["mobile", "moneypot", "monitor"],
            vec!["mobile", "moneypot", "monitor"],
            vec!["mouse", "mousepad"],
            vec!["mouse", "mousepad"],
            vec!["mouse", "mousepad"],
        ];
        let expected = expected
            .iter()
            .map(|arr| arr.iter().map(|w| w.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(
            expected,
            Solution::suggested_products(products, "mouse".into())
        );
    }
}
