#[derive(Default)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

#[derive(Default)]
pub struct MyLinkedList {
    head: Option<Box<Node>>,
}

impl MyLinkedList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&self, index: i32) -> i32 {
        let mut h = &self.head;
        let mut v = -1;
        for _ in 0..=index {
            if let Some(node) = h {
                v = node.val;
                h = &node.next;
            } else {
                return -1;
            }
        }
        v
    }

    pub fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(Node {
            val,
            next: self.head.take(),
        }));
    }

    pub fn add_at_tail(&mut self, val: i32) {
        let mut h = &mut self.head;
        while let Some(n) = h {
            h = &mut n.next;
        }
        *h = Some(Box::new(Node { val, next: None }));
    }

    pub fn add_at_index(&mut self, mut index: i32, val: i32) {
        if index <= 0 {
            self.head = Some(Box::new(Node {
                val,
                next: self.head.take(),
            }));
            return;
        }
        let mut h = &mut self.head;
        while let Some(n) = h {
            index -= 1;
            if index == 0 {
                let tail = n.next.take();
                n.next = Some(Box::new(Node { val, next: tail }));
                return;
            }
            h = &mut n.next;
        }
        *h = Some(Box::new(Node { val, next: None }));
    }

    pub fn delete_at_index(&mut self, mut index: i32) {
        if index < 0 {
            return;
        }
        if index == 0 {
            self.head = self.head.take().and_then(|mut n| n.next.take());
            return;
        }
        let mut h = &mut self.head;
        while let Some(n) = h {
            index -= 1;
            if index == 0 {
                n.next = n.next.take().and_then(|mut t| t.next.take());
                return;
            }
            h = &mut n.next;
        }
        *h = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut list = MyLinkedList::new();
        list.add_at_head(1);
        list.add_at_tail(3);
        list.add_at_index(1, 2);
        assert_eq!(2, list.get(1));
        list.delete_at_index(1);
        assert_eq!(3, list.get(1));
    }
}
