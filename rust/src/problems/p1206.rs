use std::{cell::RefCell, rc::Rc};

use rand::prelude::*;

const MAX_LEVEL: usize = 20;
const P: f64 = 0.25;

#[derive(Default)]
struct Node {
    val: i32,
    next: [Option<Rc<RefCell<Node>>>; MAX_LEVEL],
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            val,
            ..Default::default()
        }
    }
}

pub struct Skiplist {
    head: Rc<RefCell<Node>>,
    level: usize,
}

#[allow(clippy::assigning_clones)]
impl Skiplist {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            head: Rc::new(RefCell::new(Node::new(-1))),
            level: 0,
        }
    }

    pub fn search(&self, target: i32) -> bool {
        let mut curr = self.head.clone();
        for i in (0..self.level).rev() {
            while curr.borrow().next[i].is_some()
                && curr.borrow().next[i].as_ref().unwrap().borrow().val < target
            {
                let n = curr.borrow().next[i].as_ref().unwrap().clone();
                curr = n;
            }
        }
        let n = &curr.borrow().next[0];
        if let Some(n) = n {
            n.borrow().val == target
        } else {
            false
        }
    }

    pub fn add(&mut self, num: i32) {
        let mut nodes = vec![self.head.clone(); MAX_LEVEL];
        let mut curr = self.head.clone();
        for i in (0..self.level).rev() {
            while curr.borrow().next[i].is_some()
                && curr.borrow().next[i].as_ref().unwrap().borrow().val < num
            {
                let n = curr.borrow().next[i].as_ref().unwrap().clone();
                curr = n;
            }
            nodes[i] = curr.clone();
        }
        let new_level = Self::random_level();
        self.level = self.level.max(new_level);
        let node = Rc::new(RefCell::new(Node::new(num)));
        for (i, n) in nodes.iter().enumerate().take(new_level) {
            node.borrow_mut().next[i] = n.borrow().next[i].clone();
            n.borrow_mut().next[i] = Some(node.clone());
        }
    }

    pub fn erase(&mut self, num: i32) -> bool {
        let mut nodes = vec![self.head.clone(); MAX_LEVEL];
        let mut curr = self.head.clone();
        for i in (0..self.level).rev() {
            while curr.borrow().next[i].is_some()
                && curr.borrow().next[i].as_ref().unwrap().borrow().val < num
            {
                let n = curr.borrow().next[i].as_ref().unwrap().clone();
                curr = n;
            }
            nodes[i] = curr.clone();
        }
        let node = curr.borrow().next[0].clone();
        if let Some(n) = &node {
            if n.borrow().val != num {
                return false;
            }
        } else {
            return false;
        }
        let node = node.as_ref().unwrap();
        for (i, n) in nodes.iter().enumerate().take(self.level) {
            if let Some(next) = &n.borrow().next[i] {
                if !Rc::ptr_eq(next, node) {
                    break;
                }
            } else {
                break;
            }
            n.borrow_mut().next[i] = node.borrow().next[i].clone();
        }
        while self.level > 1 && self.head.borrow().next[self.level - 1].is_none() {
            self.level -= 1;
        }
        true
    }

    fn random_level() -> usize {
        let mut result = 1;
        while random::<f64>() < P && result < MAX_LEVEL {
            result += 1;
        }
        result
    }
}

// use rand::prelude::*;

// const MAX_LEVEL: usize = 32;
// const P: f64 = 0.25;

// struct Node {
//     val: i32,
//     next: [*mut Node; MAX_LEVEL],
// }

// impl Node {
//     fn new(val: i32) -> Self {
//         Self {
//             val,
//             next: [std::ptr::null_mut(); MAX_LEVEL],
//         }
//     }
// }

// pub struct Skiplist {
//     nodes: Vec<Node>,
//     head: *mut Node,
//     level: usize,
// }

// impl Skiplist {
//     #[allow(clippy::new_without_default)]
//     pub fn new() -> Self {
//         let mut nodes = Vec::with_capacity(50000);
//         nodes.push(Node::new(-1));
//         let head: *mut Node = &mut nodes[0];
//         Self {
//             nodes,
//             head,
//             level: 0,
//         }
//     }

//     pub fn search(&self, target: i32) -> bool {
//         let mut curr = self.head;
//         for i in (0..self.level).rev() {
//             unsafe {
//                 let mut next = (*curr).next[i];
//                 while !next.is_null() && (*next).val < target {
//                     curr = next;
//                     next = (*curr).next[i];
//                 }
//             }
//         }
//         unsafe {
//             let p = (*curr).next[0];
//             if !p.is_null() {
//                 (*p).val == target
//             } else {
//                 false
//             }
//         }
//     }

//     pub fn add(&mut self, num: i32) {
//         let mut update = [self.head; MAX_LEVEL];
//         let mut curr = self.head;
//         for i in (0..self.level).rev() {
//             unsafe {
//                 let mut next = (*curr).next[i];
//                 while !next.is_null() && (*next).val < num {
//                     curr = next;
//                     next = (*curr).next[i];
//                 }
//                 update[i] = curr;
//             }
//         }
//         let new_level = Self::random_level();
//         self.level = self.level.max(new_level);
//         self.nodes.push(Node::new(num));
//         let n: *mut Node = self.nodes.last_mut().unwrap();

//         for (i, &up) in update.iter().enumerate().take(new_level) {
//             unsafe {
//                 (*n).next[i] = (*up).next[i];
//                 (*up).next[i] = n;
//             }
//         }
//     }

//     pub fn erase(&mut self, num: i32) -> bool {
//         let mut nodes = [std::ptr::null_mut(); MAX_LEVEL];
//         let mut curr = self.head;
//         for i in (0..self.level).rev() {
//             unsafe {
//                 let mut next = (*curr).next[i];
//                 while !next.is_null() && (*next).val < num {
//                     curr = next;
//                     next = (*curr).next[i];
//                 }
//                 nodes[i] = curr;
//             }
//         }
//         unsafe {
//             let curr = (*curr).next[0];
//             if !curr.is_null() {
//                 if (*curr).val != num {
//                     return false;
//                 }
//             } else {
//                 return false;
//             }
//             for (i, &n) in nodes.iter().enumerate().take(self.level) {
//                 if (*n).next[i] != curr {
//                     break;
//                 }
//                 (*n).next[i] = (*curr).next[i];
//             }
//             while self.level > 1 && (*self.head).next[self.level - 1].is_null() {
//                 self.level -= 1;
//             }
//         }
//         true
//     }

//     fn random_level() -> usize {
//         let mut result = 1;
//         while random::<f64>() < P && result < MAX_LEVEL {
//             result += 1;
//         }
//         result
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut sl = Skiplist::new();
        sl.add(1);
        sl.add(2);
        sl.add(3);
        assert!(!sl.search(0));
        sl.add(4);
        assert!(sl.search(1));
        assert!(!sl.erase(0));
        assert!(sl.erase(1));
        assert!(!sl.search(1));
    }
}
