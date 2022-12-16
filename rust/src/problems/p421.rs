pub struct Solution;

#[derive(Default)]
struct Trie {
    left: Option<Box<Trie>>,
    right: Option<Box<Trie>>,
}

impl Trie {
    fn insert(&mut self, num: i32) {
        let mut t = self;
        for i in (0..=30).rev() {
            let bit = (num >> i) & 1;
            if bit == 0 {
                t.left = Some(t.left.take().unwrap_or_default());
                t = t.left.as_mut().unwrap();
            } else {
                t.right = Some(t.right.take().unwrap_or_default());
                t = t.right.as_mut().unwrap();
            }
        }
    }

    fn search(&self, prefix: i32) -> i32 {
        let mut t = self;
        let mut result = 0;
        for i in (0..=30).rev() {
            let bit = (prefix >> i) & 1;
            if bit == 0 {
                if let Some(right) = &t.right {
                    result = result * 2 + 1;
                    t = right.as_ref();
                } else {
                    result *= 2;
                    t = t.left.as_ref().unwrap().as_ref();
                }
            } else if let Some(left) = &t.left {
                result = result * 2 + 1;
                t = left.as_ref();
            } else {
                result *= 2;
                t = t.right.as_ref().unwrap().as_ref();
            }
        }
        result
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut result = 0i32;
        let mut trie = Trie::default();
        for w in nums.windows(2) {
            trie.insert(w[0]);
            result = result.max(trie.search(w[1]));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(28, Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            127,
            Solution::find_maximum_xor(vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70])
        );
    }
}
