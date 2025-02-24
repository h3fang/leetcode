use std::collections::VecDeque;

pub struct OrderedStream {
    frags: VecDeque<String>,
    ptr: i32,
}

impl OrderedStream {
    pub fn new(n: i32) -> Self {
        Self {
            frags: (0..n).map(|_| String::new()).collect(),
            ptr: 0,
        }
    }

    pub fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let mut result = vec![];
        if id_key - 1 < self.ptr {
            result
        } else {
            self.frags[(id_key - 1 - self.ptr) as usize] = value;
            while self.frags.front().is_some_and(|s| !s.is_empty()) {
                result.push(self.frags.pop_front().unwrap());
                self.ptr += 1;
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut os = OrderedStream::new(5);
        assert!(os.insert(3, "ccccc".to_string()).is_empty());
        assert_eq!(
            vec![String::from("aaaaa")],
            os.insert(1, "aaaaa".to_string())
        );
        assert_eq!(
            vec![String::from("bbbbb"), String::from("ccccc")],
            os.insert(2, "bbbbb".to_string())
        );
        assert!(os.insert(5, "eeeee".to_string()).is_empty());
        assert_eq!(
            vec![String::from("ddddd"), String::from("eeeee")],
            os.insert(4, "ddddd".to_string())
        );
    }
}
