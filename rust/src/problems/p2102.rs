use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, Default, PartialEq, Eq)]
struct Site {
    name: String,
    score: i32,
}

impl PartialOrd for Site {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Site {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.score.cmp(&other.score) {
            std::cmp::Ordering::Equal => other.name.cmp(&self.name),
            ord => ord,
        }
    }
}

#[derive(Debug, Default)]
pub struct SORTracker {
    k: usize,
    left: BinaryHeap<Reverse<Site>>,
    right: BinaryHeap<Site>,
}

impl SORTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: String, score: i32) {
        let s = Site { name, score };
        if self.left.len() < self.k + 1 {
            self.left.push(Reverse(s));
        } else if self.left.peek().unwrap().0 < s {
            let small = self.left.pop().unwrap();
            self.left.push(Reverse(s));
            self.right.push(small.0);
        } else {
            self.right.push(s);
        }

        println!("{}", self);
    }

    pub fn get(&mut self) -> String {
        let r = self.left.peek().unwrap().0.name.clone();
        if let Some(s) = self.right.pop() {
            self.left.push(Reverse(s));
        }
        self.k += 1;
        r
    }
}

impl std::fmt::Display for SORTracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.left.iter().rev() {
            write!(f, "{:?}", s)?
        }
        for s in &self.right {
            write!(f, "{:?}", s)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut st = SORTracker::new();
        st.add("bradford".to_string(), 2);
        st.add("branford".to_string(), 3);
        assert_eq!("branford", st.get());
        st.add("alps".to_string(), 2);
        assert_eq!("alps", st.get());
        st.add("orland".to_string(), 2);
        assert_eq!("bradford", st.get());
        st.add("orlando".to_string(), 3);
        assert_eq!("bradford", st.get());
        st.add("alpine".to_string(), 2);
        assert_eq!("bradford", st.get());
        assert_eq!("orland", st.get());
    }
}
