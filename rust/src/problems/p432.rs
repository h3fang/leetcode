use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct DLNode {
    count: usize,
    strs: HashSet<String>,
    prev: Option<usize>,
    next: Option<usize>,
}

#[derive(Default)]
pub struct AllOne {
    nodes: Vec<DLNode>,
    map: HashMap<String, usize>,
    head: Option<usize>,
    tail: Option<usize>,
}

impl AllOne {
    pub fn new() -> Self {
        Default::default()
    }

    fn remove_key(&mut self, i: usize, key: &str) {
        self.nodes[i].strs.remove(key);
        if self.nodes[i].strs.is_empty() {
            if let Some(prev) = self.nodes[i].prev {
                self.nodes[prev].next = self.nodes[i].next;
            } else {
                self.head = self.nodes[i].next;
            }

            if let Some(next) = self.nodes[i].next {
                self.nodes[next].prev = self.nodes[i].prev;
            } else {
                self.tail = self.nodes[i].prev;
            }
        }
    }

    fn new_node(&mut self, count: usize) -> usize {
        let c = self.nodes.len();
        self.nodes.push(DLNode {
            count,
            ..Default::default()
        });
        c
    }

    pub fn inc(&mut self, key: String) {
        if let Some(&i) = self.map.get(&key) {
            let cnt = self.nodes[i].count + 1;

            let mut cur = usize::MAX;

            if let Some(next) = self.nodes[i].next {
                if self.nodes[next].count == cnt {
                    cur = next;
                }
            }
            if cur == usize::MAX {
                cur = self.new_node(cnt);
                if let Some(next) = self.nodes[i].next {
                    self.nodes[cur].next = Some(next);
                    self.nodes[next].prev = Some(cur);
                }
                self.nodes[i].next = Some(cur);
                self.nodes[cur].prev = Some(i);
                if self.tail == Some(i) {
                    self.tail = Some(cur);
                }
            }

            self.nodes[cur].strs.insert(key.clone());
            self.remove_key(i, &key);
            self.map.insert(key, cur);
        } else {
            let mut cur = usize::MAX;
            if let Some(h) = self.head {
                if self.nodes[h].count == 1 {
                    cur = h;
                }
            }
            if cur == usize::MAX {
                cur = self.new_node(1);
                self.nodes[cur].next = self.head;
                if let Some(h) = self.head {
                    self.nodes[h].prev = Some(cur);
                }
            }
            self.nodes[cur].strs.insert(key.clone());
            if self.tail.is_none() {
                self.tail = Some(cur);
            }
            self.head = Some(cur);
            self.map.insert(key, cur);
        }
    }

    pub fn dec(&mut self, key: String) {
        let i = *self.map.get(&key).unwrap();
        let cnt = self.nodes[i].count - 1;
        if cnt == 0 {
            self.remove_key(i, &key);
            self.map.remove(&key);
            return;
        }

        let mut cur = usize::MAX;

        if let Some(prev) = self.nodes[i].prev {
            if self.nodes[prev].count == cnt {
                cur = prev;
            }
        }
        if cur == usize::MAX {
            cur = self.new_node(cnt);
            if let Some(prev) = self.nodes[i].prev {
                self.nodes[prev].next = Some(cur);
                self.nodes[cur].prev = Some(prev);
            }
            self.nodes[cur].next = Some(i);
            self.nodes[i].prev = Some(cur);
            if self.head == Some(i) {
                self.head = Some(cur);
            }
        }

        self.nodes[cur].strs.insert(key.clone());
        self.remove_key(i, &key);
        self.map.insert(key, cur);
    }

    pub fn get_max_key(&self) -> String {
        if let Some(i) = self.tail {
            self.nodes[i].strs.iter().next().unwrap().to_string()
        } else {
            String::new()
        }
    }

    pub fn get_min_key(&self) -> String {
        if let Some(i) = self.head {
            self.nodes[i].strs.iter().next().unwrap().to_string()
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut ao = AllOne::new();
        ao.inc("hello".to_string());
        ao.inc("hello".to_string());
        assert_eq!("hello", ao.get_max_key());
        assert_eq!("hello", ao.get_min_key());
        ao.inc("leet".to_string());
        assert_eq!("hello", ao.get_max_key());
        assert_eq!("leet", ao.get_min_key());
        ao.dec("hello".to_string());
        ao.dec("hello".to_string());
        assert_eq!("leet", ao.get_max_key());
        assert_eq!("leet", ao.get_min_key());
    }

    #[test]
    fn case2() {
        let mut ao = AllOne::new();
        ao.inc("hello".to_string());
        ao.inc("goodbye".to_string());
        assert!(["hello", "goodbye"].contains(&ao.get_max_key().as_str()));
        ao.inc("leet".to_string());
        ao.inc("code".to_string());
        ao.inc("leet".to_string());
        ao.dec("hello".to_string());
        ao.inc("leet".to_string());
        ao.inc("code".to_string());
        ao.inc("code".to_string());
        assert!(["leet", "code"].contains(&ao.get_max_key().as_str()));
    }
}
