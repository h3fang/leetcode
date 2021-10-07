use std::fmt;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn append(&mut self, val: i32) {
        match self.next {
            Some(ref mut next) => next.append(val),
            None => {
                let node = Some(Box::new(ListNode::new(val)));
                self.next = node;
            }
        }
    }

    pub fn from_vec(vec: &[i32]) -> Option<Box<Self>> {
        let mut head = ListNode::new(0);
        for n in vec {
            head.append(*n);
        }
        head.next
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = write!(f, "{}", self.val);
        match self.next {
            Some(ref next) => {
                write!(f, "->")?;
                next.fmt(f)
            }
            None => r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 2, 3, 4, 5];
        let head = ListNode::from_vec(&nums);

        assert_eq!("1->2->3->4->5", head.as_deref().unwrap().to_string());

        let mut h = &head;
        for n in nums {
            match h {
                Some(node) => {
                    assert_eq!(node.val, n);
                    h = &node.next;
                }
                None => panic!("test failed"),
            }
        }
        assert_eq!(h, &None);
    }
}
