use std::collections::VecDeque;

#[derive(Default)]
pub struct TextEditor {
    left: VecDeque<u8>,
    right: VecDeque<u8>,
}

impl TextEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_text(&mut self, text: String) {
        self.left.extend(text.bytes());
    }

    pub fn delete_text(&mut self, k: i32) -> i32 {
        let deleted = (self.left.len() as i32).min(k);
        self.left.truncate(self.left.len() - deleted as usize);
        deleted
    }

    pub fn cursor_left(&mut self, k: i32) -> String {
        let n = self.left.len().min(k as usize);
        for _ in 0..n {
            let c = self.left.pop_back().unwrap();
            self.right.push_front(c);
        }
        let i = self.left.len().saturating_sub(10);
        String::from_utf8(self.left.range(i..).cloned().collect()).unwrap()
    }

    pub fn cursor_right(&mut self, k: i32) -> String {
        let n = self.right.len().min(k as usize);
        for _ in 0..n {
            let c = self.right.pop_front().unwrap();
            self.left.push_back(c);
        }
        let i = self.left.len().saturating_sub(10);
        String::from_utf8(self.left.range(i..).cloned().collect()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut e = TextEditor::new();
        e.add_text("leetcode".to_owned());
        assert_eq!(4, e.delete_text(4));
        e.add_text("practice".to_owned());
        assert_eq!("etpractice", e.cursor_right(3));
        assert_eq!("leet", e.cursor_left(8));
        assert_eq!(4, e.delete_text(10));
        assert_eq!("", e.cursor_left(2));
        assert_eq!("practi", e.cursor_right(6));
    }
}
