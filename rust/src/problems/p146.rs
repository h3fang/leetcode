use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct Node {
    key: i32,
    value: i32,
    prev: *mut Node,
    next: *mut Node,
}

pub struct LRUCache {
    dummy_head: Box<Node>,
    dummy_tail: Box<Node>,
    map: HashMap<i32, Box<Node>>,
    capacity: usize,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let mut head = Box::new(Node {
            key: -1,
            value: -1,
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        });
        let mut tail = Box::new(Node {
            key: -1,
            value: -1,
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        });
        head.next = tail.as_mut();
        tail.prev = head.as_mut();
        Self {
            dummy_head: head,
            dummy_tail: tail,
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
                let head = self.dummy_head.next;
                self.dummy_head.next = node;
                node.prev = self.dummy_head.as_mut();
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
                let head = self.dummy_head.next;
                self.dummy_head.next = node.as_mut();
                node.prev = self.dummy_head.as_mut();
                (*head).prev = node.as_mut();
                node.next = head;
            }
        } else {
            if self.map.len() == self.capacity {
                let tail = self.dummy_tail.prev;
                unsafe {
                    (*(*tail).prev).next = self.dummy_tail.as_mut();
                    self.dummy_tail.prev = (*tail).prev;
                    self.map.remove(&(*tail).key);
                }
            }
            let mut node = Box::new(Node {
                key,
                value,
                prev: self.dummy_head.as_mut(),
                next: self.dummy_head.next,
            });
            let n: *mut Node = node.as_mut();
            self.map.insert(key, node);
            unsafe {
                (*self.dummy_head.next).prev = n;
                self.dummy_head.next = n;
            }
        }
    }
}

#[derive(Clone)]
struct SafeNode {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<SafeNode>>>,
    next: Option<Rc<RefCell<SafeNode>>>,
}

#[derive(Default)]
pub struct SafeLRUCache {
    dummy_head: Option<Rc<RefCell<SafeNode>>>,
    dummy_tail: Option<Rc<RefCell<SafeNode>>>,
    map: HashMap<i32, Rc<RefCell<SafeNode>>>,
    capacity: usize,
}

#[allow(clippy::assigning_clones)]
impl SafeLRUCache {
    pub fn new(capacity: i32) -> Self {
        let head = Rc::new(RefCell::new(SafeNode {
            key: -1,
            value: -1,
            prev: None,
            next: None,
        }));
        let tail = Rc::new(RefCell::new(SafeNode {
            key: -1,
            value: -1,
            prev: None,
            next: None,
        }));
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());
        Self {
            dummy_head: Some(head),
            dummy_tail: Some(tail),
            capacity: capacity as usize,
            ..Default::default()
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get_mut(&key) {
            // is head
            if node.borrow().prev.as_ref().unwrap().borrow().key == -1 {
                return node.borrow().value;
            }

            // remove node
            node.borrow().prev.as_ref().unwrap().borrow_mut().next = node.borrow().next.clone();
            node.borrow().next.as_ref().unwrap().borrow_mut().prev = node.borrow().prev.clone();

            // insert head
            let h = self.dummy_head.as_ref().unwrap().borrow_mut().next.take();
            self.dummy_head.as_ref().unwrap().borrow_mut().next = Some(node.clone());
            node.borrow_mut().prev = self.dummy_head.clone();
            h.as_ref().unwrap().borrow_mut().prev = Some(node.clone());
            node.borrow_mut().next = h;

            node.borrow().value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get_mut(&key) {
            node.borrow_mut().value = value;

            // is head
            if node.borrow().prev.as_ref().unwrap().borrow().key == -1 {
                return;
            }

            // remove node
            node.borrow().prev.as_ref().unwrap().borrow_mut().next = node.borrow().next.clone();
            node.borrow().next.as_ref().unwrap().borrow_mut().prev = node.borrow().prev.clone();

            // insert head
            let h = self.dummy_head.as_ref().unwrap().borrow_mut().next.take();
            self.dummy_head.as_ref().unwrap().borrow_mut().next = Some(node.clone());
            node.borrow_mut().prev = self.dummy_head.clone();
            h.as_ref().unwrap().borrow_mut().prev = Some(node.clone());
            node.borrow_mut().next = h;
        } else {
            if self.map.len() == self.capacity {
                let tail = self.dummy_tail.as_ref().unwrap().borrow_mut().prev.take();
                self.map.remove(&tail.as_ref().unwrap().borrow().key);
                tail.as_ref()
                    .unwrap()
                    .borrow_mut()
                    .prev
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .next = self.dummy_tail.clone();
                self.dummy_tail.as_ref().unwrap().borrow_mut().prev =
                    tail.as_ref().unwrap().borrow_mut().prev.clone();
            }
            let node = Rc::new(RefCell::new(SafeNode {
                key,
                value,
                prev: self.dummy_head.clone(),
                next: self.dummy_head.as_ref().unwrap().borrow().next.clone(),
            }));
            self.map.insert(key, node.clone());
            self.dummy_head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow_mut()
                .prev = Some(node.clone());
            self.dummy_head.as_ref().unwrap().borrow_mut().next = Some(node);
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

        let mut lru = SafeLRUCache::new(2);
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

        let mut lru = SafeLRUCache::new(2);
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

        let mut lru = SafeLRUCache::new(2);
        lru.put(2, 1);
        lru.put(1, 1);
        lru.put(2, 3);
        lru.put(4, 1);
        assert_eq!(-1, lru.get(1));
        assert_eq!(3, lru.get(2));
    }
}
