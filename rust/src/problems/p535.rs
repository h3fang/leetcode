use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;

#[derive(Default)]
pub struct Codec {
    long_to_short: HashMap<String, String>,
    short_to_long: HashMap<String, String>,
}

impl Codec {
    pub fn new() -> Self {
        Default::default()
    }

    fn random_string(len: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    fn next_short(&self) -> String {
        let mut s: String = Self::random_string(10);
        while self.short_to_long.contains_key(&s) {
            s = Self::random_string(10);
        }
        s
    }

    pub fn encode(&mut self, long_url: String) -> String {
        if let Some(s) = self.long_to_short.get(&long_url) {
            s.to_string()
        } else {
            let s = self.next_short();
            self.long_to_short.insert(long_url.clone(), s.clone());
            self.short_to_long.insert(s.clone(), long_url);
            s
        }
    }

    pub fn decode(&self, short_url: String) -> String {
        self.short_to_long
            .get(&short_url)
            .cloned()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn case1() {
        let mut codec = Codec::new();

        let long = (0..100)
            .map(|_| Codec::random_string(thread_rng().gen_range(10..100)))
            .collect::<Vec<_>>();
        let short = long
            .iter()
            .map(|s| codec.encode(s.to_string()))
            .collect::<Vec<_>>();
        for (l, s) in long.iter().zip(&short) {
            assert_eq!(s.as_str(), codec.encode(l.to_string()));
            assert_eq!(l.as_str(), codec.decode(s.to_string()));
        }
    }
}
