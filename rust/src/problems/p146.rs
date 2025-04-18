use std::collections::HashMap;

struct Node {
    key: i32,
    value: i32,
    prev: *mut Node,
    next: *mut Node,
}

pub struct LRUCache {
    dummy: Box<Node>,
    map: HashMap<i32, Box<Node>>,
    capacity: usize,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let mut dummy = Box::new(Node {
            key: -1,
            value: -1,
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        });
        dummy.next = dummy.as_mut();
        dummy.prev = dummy.as_mut();
        Self {
            dummy,
            map: HashMap::new(),
            capacity: capacity as usize,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get_mut(&key) {
            unsafe {
                let node = node.as_mut();
                // is head
                if (*node.prev).key == -1 {
                    return node.value;
                }

                // remove node
                (*node.prev).next = node.next;
                (*node.next).prev = node.prev;

                // insert head
                let head = self.dummy.next;
                self.dummy.next = node;
                node.prev = self.dummy.as_mut();
                (*head).prev = node;
                node.next = head;

                node.value
            }
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get_mut(&key) {
            unsafe {
                node.value = value;
                // is head
                if (*node.prev).key == -1 {
                    return;
                }

                // remove node
                (*node.prev).next = node.next;
                (*node.next).prev = node.prev;

                // insert head
                let head = self.dummy.next;
                self.dummy.next = node.as_mut();
                node.prev = self.dummy.as_mut();
                (*head).prev = node.as_mut();
                node.next = head;
            }
        } else {
            if self.map.len() == self.capacity {
                let tail = self.dummy.prev;
                unsafe {
                    (*(*tail).prev).next = self.dummy.as_mut();
                    self.dummy.prev = (*tail).prev;
                    self.map.remove(&(*tail).key);
                }
            }
            let mut node = Box::new(Node {
                key,
                value,
                prev: self.dummy.as_mut(),
                next: self.dummy.next,
            });
            let n: *mut Node = node.as_mut();
            self.map.insert(key, node);
            unsafe {
                (*self.dummy.next).prev = n;
                self.dummy.next = n;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut lru = LRUCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        assert_eq!(1, lru.get(1));
        lru.put(3, 3);
        assert_eq!(-1, lru.get(2));
        lru.put(4, 4);
        assert_eq!(-1, lru.get(1));
        assert_eq!(3, lru.get(3));
        assert_eq!(4, lru.get(4));
    }

    #[test]
    fn case2() {
        let mut lru = LRUCache::new(2);
        lru.put(1, 0);
        lru.put(2, 2);
        assert_eq!(0, lru.get(1));
        lru.put(3, 3);
        assert_eq!(-1, lru.get(2));
        lru.put(4, 4);
        assert_eq!(-1, lru.get(1));
        assert_eq!(3, lru.get(3));
        assert_eq!(4, lru.get(4));
    }

    #[test]
    fn case3() {
        let mut lru = LRUCache::new(2);
        lru.put(2, 1);
        lru.put(1, 1);
        lru.put(2, 3);
        lru.put(4, 1);
        assert_eq!(-1, lru.get(1));
        assert_eq!(3, lru.get(2));
    }
}
