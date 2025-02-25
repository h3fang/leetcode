use std::{cell::RefCell, rc::Rc};

const MAX_HEIGHT: usize = 32;

#[derive(Default)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
    down: Option<Rc<RefCell<Node>>>,
}

pub struct Skiplist {
    head: Option<Rc<RefCell<Node>>>,
}

impl Skiplist {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            head: Some(Rc::new(RefCell::new(Node {
                val: -1,
                ..Default::default()
            }))),
        }
    }

    fn jump(&self, val: i32) -> Vec<Rc<RefCell<Node>>> {
        let mut nodes = Vec::with_capacity(MAX_HEIGHT);
        let mut curr = self.head.clone();
        while let Some(mut c) = curr {
            while c
                .borrow()
                .next
                .as_ref()
                .is_some_and(|n| n.borrow().val < val)
            {
                let next = c.borrow().next.clone().unwrap();
                c = next;
            }
            curr = c.borrow().down.clone();
            nodes.push(c);
        }
        nodes
    }

    pub fn search(&self, target: i32) -> bool {
        let nodes = self.jump(target);
        nodes.last().is_some_and(|n| {
            n.borrow()
                .next
                .as_ref()
                .is_some_and(|next| next.borrow().val == target)
        })
    }

    pub fn add(&mut self, num: i32) {
        let mut nodes = self.jump(num);
        let mut prev = None;
        for i in 0..MAX_HEIGHT {
            if i > 0 && rand::random::<f32>() < 0.5 {
                return;
            }
            if let Some(n) = nodes.pop() {
                prev = Some(Rc::new(RefCell::new(Node {
                    val: num,
                    next: n.borrow().next.clone(),
                    down: prev,
                })));
                n.borrow_mut().next = prev.clone();
            } else {
                prev = Some(Rc::new(RefCell::new(Node {
                    val: num,
                    next: None,
                    down: prev,
                })));
                self.head = Some(Rc::new(RefCell::new(Node {
                    val: -1,
                    next: prev.clone(),
                    down: self.head.take(),
                })));
            }
        }
    }

    pub fn erase(&mut self, num: i32) -> bool {
        let nodes = self.jump(num);
        let last = nodes.last().unwrap();
        let next = last.borrow().next.clone();
        if next.is_none() || next.unwrap().borrow().val != num {
            return false;
        }
        for n in nodes.into_iter() {
            let mut next = n.borrow_mut().next.take();
            if next.as_ref().is_some_and(|n| n.borrow().val == num) {
                next = next.unwrap().borrow_mut().next.take();
            }
            n.borrow_mut().next = next;
        }
        true
    }
}

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
