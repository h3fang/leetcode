pub struct Solution;

#[derive(Default, Clone, Copy)]
struct Node {
    pre: i32,
    suf: i32,
    max: i32,
}

struct SegmentTree {
    s: Vec<u8>,
    t: Vec<Node>,
}

impl SegmentTree {
    fn new(s: Vec<u8>) -> Self {
        let n = s.len();
        let w = usize::BITS - (n - 1).leading_zeros();
        let t = vec![Node::default(); 2 << w];
        let mut tree = Self { s, t };
        tree.build(1, 0, tree.s.len() as i32 - 1);
        tree
    }

    fn build(&mut self, i: usize, l: i32, r: i32) {
        if l == r {
            self.t[i] = Node {
                pre: 1,
                suf: 1,
                max: 1,
            };
            return;
        }
        let m = l.midpoint(r);
        self.build(2 * i, l, m);
        self.build(2 * i + 1, m + 1, r);
        self.maintain(i, l, r, m);
    }

    fn maintain(&mut self, i: usize, l: i32, r: i32, m: i32) {
        let left = &self.t[2 * i];
        let right = &self.t[2 * i + 1];

        let mut max = left.max.max(right.max);
        let mut pre = left.pre;
        let mut suf = right.suf;

        if self.s[m as usize] == self.s[m as usize + 1] {
            max = max.max(left.suf + right.pre);
            if pre == m - l + 1 {
                pre += right.pre;
            }
            if suf == r - m {
                suf += left.suf;
            }
        }

        self.t[i] = Node { pre, suf, max };
    }

    fn update(&mut self, idx: i32, v: u8) {
        fn update_impl(tree: &mut SegmentTree, i: usize, l: i32, r: i32, idx: i32, v: u8) {
            if l == r {
                tree.s[idx as usize] = v;
                return;
            }

            let m = l.midpoint(r);
            if idx <= m {
                update_impl(tree, 2 * i, l, m, idx, v);
            } else {
                update_impl(tree, 2 * i + 1, m + 1, r, idx, v);
            }
            tree.maintain(i, l, r, m);
        }
        update_impl(self, 1, 0, self.s.len() as i32 - 1, idx, v);
    }
}

impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        let mut tree = SegmentTree::new(s.into_bytes());
        let mut ans = Vec::with_capacity(query_characters.len());
        for (c, i) in query_characters.bytes().zip(query_indices) {
            tree.update(i, c);
            ans.push(tree.t[1].max);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![3, 3, 4],
            Solution::longest_repeating("babacc".to_string(), "bcb".to_string(), vec![1, 3, 3])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![2, 3],
            Solution::longest_repeating("abyzz".to_string(), "aa".to_string(), vec![2, 1])
        );
    }
}
