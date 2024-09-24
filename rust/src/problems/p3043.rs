pub struct Solution;

#[derive(Default)]
struct Trie {
    next: [Option<Box<Trie>>; 10],
}

fn digits_rev(mut x: i32) -> Vec<i32> {
    let mut digits = Vec::with_capacity(9);
    loop {
        digits.push(x % 10);
        if x >= 10 {
            x /= 10;
        } else {
            return digits;
        }
    }
}

impl Trie {
    fn new(arr: Vec<i32>) -> Self {
        let mut t = Self::default();
        for x in arr {
            t.insert(x);
        }
        t
    }

    fn insert(&mut self, x: i32) {
        let digits = digits_rev(x);
        let mut t = self;
        for d in digits.into_iter().rev() {
            t = t.next[d as usize].get_or_insert_with(Default::default);
        }
    }

    fn search(&self, x: i32) -> i32 {
        let digits = digits_rev(x);
        let mut t = self;
        for (i, &d) in digits.iter().rev().enumerate() {
            if let Some(n) = &t.next[d as usize] {
                t = n;
            } else {
                return i as i32;
            }
        }
        digits.len() as i32
    }
}

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let t = Trie::new(arr1);
        arr2.into_iter().map(|x| t.search(x)).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::longest_common_prefix(vec![1, 10, 100], vec![1000])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::longest_common_prefix(vec![1, 2, 3], vec![4, 4, 4])
        );
    }
}
