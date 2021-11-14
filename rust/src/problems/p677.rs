#[derive(Default)]
pub struct MapSum {
    next: [Option<Box<MapSum>>; 26],
    val: i32,
    sum: i32,
}

impl MapSum {
    pub fn new() -> Self {
        Self::default()
    }

    fn insert_impl(&mut self, key: &[u8], val: i32) -> i32 {
        if key.is_empty() {
            let delta = val - self.val;
            self.val = val;
            self.sum += delta;
            delta
        } else {
            let i = (key[0] - b'a') as usize;
            let delta = match &mut self.next[i] {
                Some(next) => next.insert_impl(&key[1..], val),
                None => {
                    let mut next = Self::new();
                    let delta = next.insert_impl(&key[1..], val);
                    self.next[i] = Some(Box::new(next));
                    delta
                }
            };
            self.sum += delta;
            delta
        }
    }

    pub fn insert(&mut self, key: String, val: i32) {
        self.insert_impl(key.as_bytes(), val);
    }

    fn sum_impl(&self, prefix: &[u8]) -> i32 {
        if prefix.is_empty() {
            self.sum
        } else {
            let i = (prefix[0] - b'a') as usize;
            match &self.next[i] {
                Some(next) => next.sum_impl(&prefix[1..]),
                None => 0,
            }
        }
    }

    pub fn sum(&self, prefix: String) -> i32 {
        self.sum_impl(prefix.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut ms = MapSum::new();
        ms.insert("apple".to_string(), 3);
        assert_eq!(3, ms.sum("ap".to_string()));
        ms.insert("app".to_string(), 2);
        assert_eq!(5, ms.sum("ap".to_string()));
    }
}
