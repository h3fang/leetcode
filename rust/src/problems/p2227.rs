use std::collections::HashMap;

pub struct Encrypter {
    key_value: HashMap<char, [u8; 2]>,
    dictionary: HashMap<String, i32>,
}

impl Encrypter {
    pub fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self {
        let mut key_value = HashMap::new();
        for (i, &key) in keys.iter().enumerate() {
            let v = values[i].as_bytes();
            let v = [v[0], v[1]];
            key_value.insert(key, v);
        }
        let mut encrypter = Self {
            key_value,
            dictionary: HashMap::new(),
        };
        for s in dictionary {
            let encrypted = encrypter.encrypt(s);
            *encrypter.dictionary.entry(encrypted).or_default() += 1;
        }
        encrypter
    }

    pub fn encrypt(&self, word1: String) -> String {
        let mut result = String::with_capacity(2 * word1.len());
        for c in word1.chars() {
            let v = self.key_value.get(&c).unwrap();
            result.push(v[0] as char);
            result.push(v[1] as char);
        }
        result
    }

    pub fn decrypt(&self, word2: String) -> i32 {
        self.dictionary.get(&word2).cloned().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let values = ["ei", "zf", "ei", "am"];
        let values = values.iter().map(|v| v.to_string()).collect();
        let dictionary = [
            "abcd", "acbd", "adbc", "badc", "dacb", "cadb", "cbda", "abad",
        ];
        let dictionary = dictionary.iter().map(|v| v.to_string()).collect();
        let e = Encrypter::new(vec!['a', 'b', 'c', 'd'], values, dictionary);
        assert_eq!("eizfeiam", e.encrypt("abcd".to_string()));
        assert_eq!(2, e.decrypt("eizfeiam".to_string()));
    }

    #[test]
    fn case2() {
        let values = ["aa", "bb", "cc", "zz"];
        let values = values.iter().map(|v| v.to_string()).collect();
        let dictionary = ["aa", "aaa", "aaaa", "aaaaa", "aaaaaaa"];
        let dictionary = dictionary.iter().map(|v| v.to_string()).collect();
        let e = Encrypter::new(vec!['a', 'b', 'c', 'z'], values, dictionary);
        assert_eq!(1, e.decrypt("aaaa".to_string()));
        assert_eq!(0, e.decrypt("aa".to_string()));
        assert_eq!(1, e.decrypt("aaaa".to_string()));
        assert_eq!(1, e.decrypt("aaaaaaaa".to_string()));
        assert_eq!(1, e.decrypt("aaaaaaaaaaaaaa".to_string()));
        assert_eq!(0, e.decrypt("aefagafvabfgshdthn".to_string()));
    }
}
