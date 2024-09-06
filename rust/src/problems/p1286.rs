pub struct CombinationIterator {
    chars: Vec<u8>,
    len: u32,
    mask: u32,
}

impl CombinationIterator {
    pub fn new(characters: String, combination_length: i32) -> Self {
        let mask = (1u32 << characters.len()) - 1;
        let chars = characters.into_bytes();
        Self {
            chars,
            len: combination_length as u32,
            mask,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> String {
        while self.mask > 0 && self.mask.count_ones() != self.len {
            self.mask -= 1;
        }
        let mut result = String::new();
        for i in (0..self.chars.len()).rev() {
            if ((1 << i) & self.mask) > 0 {
                result.push(self.chars[self.chars.len() - 1 - i] as char);
            }
        }
        self.mask -= 1;
        result
    }

    pub fn has_next(&mut self) -> bool {
        while self.mask > 0 && self.mask.count_ones() != self.len {
            self.mask -= 1;
        }
        self.mask > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut ci = CombinationIterator::new("abc".to_string(), 2);
        assert_eq!("ab".to_string(), ci.next());
        assert!(ci.has_next());
        assert_eq!("ac".to_string(), ci.next());
        assert!(ci.has_next());
        assert_eq!("bc".to_string(), ci.next());
        assert!(!ci.has_next());
    }
}
