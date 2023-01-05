pub struct Solution;

#[derive(Default)]
struct Trie {
    next: [Option<Box<Trie>>; 2],
    count: i32,
}

impl Trie {
    fn insert(&mut self, num: i32) {
        let mut t = self;
        for k in (0..15).rev() {
            let i = (num >> k) & 1;
            t = t.next[i as usize].get_or_insert_with(Default::default);
            t.count += 1;
        }
    }

    fn search(&self, num: i32, x: i32) -> i32 {
        let mut result = 0;
        let mut t = self;
        for k in (0..15).rev() {
            let i = (num >> k) & 1;
            if x & (1 << k) > 0 {
                if let Some(c) = &t.next[i as usize] {
                    result += c.count;
                }
                if let Some(next) = &t.next[(i ^ 1) as usize] {
                    t = next;
                } else {
                    return result;
                }
            } else if let Some(next) = &t.next[i as usize] {
                t = next;
            } else {
                return result;
            }
        }
        result += t.count;
        result
    }
}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        fn f(nums: &[i32], x: i32) -> i32 {
            let mut result = 0;
            let mut t = Trie::default();
            for w in nums.windows(2) {
                t.insert(w[0]);
                result += t.search(w[1], x);
            }
            result
        }
        f(&nums, high) - f(&nums, low - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::count_pairs(vec![1, 4, 2, 7], 2, 6));
    }

    #[test]
    fn case2() {
        assert_eq!(8, Solution::count_pairs(vec![9, 8, 4, 2, 1], 5, 14));
    }
}
