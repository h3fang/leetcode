pub struct OrderedStream {
    frags: Vec<String>,
    ptr: usize,
}

impl OrderedStream {
    pub fn new(n: i32) -> Self {
        Self {
            frags: vec![String::new(); n as usize + 1],
            ptr: 1,
        }
    }

    pub fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let mut result = vec![];
        if (id_key as usize) < self.ptr {
            result
        } else {
            self.frags[id_key as usize] = value;
            for (i, f) in self.frags.iter().enumerate().skip(self.ptr) {
                if !f.is_empty() {
                    result.push(f.to_string());
                } else {
                    self.ptr = i;
                    break;
                }
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
