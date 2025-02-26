pub struct BrowserHistory {
    current: i32,
    history: Vec<String>,
}

impl BrowserHistory {
    pub fn new(homepage: String) -> Self {
        let mut history = Vec::with_capacity(100);
        history.push(homepage);
        Self {
            current: 0,
            history,
        }
    }

    pub fn visit(&mut self, url: String) {
        self.history
            .resize(self.current as usize + 1, String::new());
        self.history.push(url);
        self.current += 1;
    }

    pub fn back(&mut self, steps: i32) -> String {
        self.current = (self.current - steps).max(0);
        self.history[self.current as usize].to_string()
    }

    pub fn forward(&mut self, steps: i32) -> String {
        self.current = (self.current + steps).min(self.history.len() as i32 - 1);
        self.history[self.current as usize].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut bh = BrowserHistory::new("leetcode.com".to_string());
        bh.visit("google.com".to_string());
        bh.visit("facebook.com".to_string());
        bh.visit("youtube.com".to_string());
        assert_eq!("facebook.com", bh.back(1));
        assert_eq!("google.com", bh.back(1));
        assert_eq!("facebook.com", bh.forward(1));
        bh.visit("linkedin.com".to_string());
        assert_eq!("linkedin.com", bh.forward(2));
        assert_eq!("google.com", bh.back(2));
        assert_eq!("leetcode.com", bh.back(7));
    }
}
