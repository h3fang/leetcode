use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub struct Solution;

impl Solution {
    pub fn min_mutation(start: String, end: String, mut bank: Vec<String>) -> i32 {
        fn is_connected(a: &str, b: &str) -> bool {
            let mut diff = 0;
            for (x, y) in a.chars().zip(b.chars()) {
                if x != y {
                    diff += 1;
                    if diff > 1 {
                        return false;
                    }
                }
            }
            diff == 1
        }

        if !bank.contains(&start) {
            bank.push(start.to_string());
        }
        let mut g: HashMap<&str, Vec<&str>> = HashMap::new();
        for (i, a) in bank.iter().enumerate() {
            for b in &bank[i + 1..] {
                if is_connected(a, b) {
                    g.entry(a).or_default().push(b);
                    g.entry(b).or_default().push(a);
                }
            }
        }
        let mut visited: HashSet<&str> = HashSet::new();
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), start.as_str()));
        while let Some((Reverse(dist), a)) = q.pop() {
            if a == end {
                return dist;
            }
            visited.insert(a);
            if let Some(children) = g.get(a) {
                for b in children {
                    if !visited.contains(b) {
                        q.push((Reverse(dist + 1), b));
                    }
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let start = "AACCGGTT".into();
        let end = "AACCGGTA".into();
        let bank = vec!["AACCGGTA".into()];
        assert_eq!(1, Solution::min_mutation(start, end, bank));
    }

    #[test]
    fn case2() {
        let start = "AACCGGTT".into();
        let end = "AAACGGTA".into();
        let bank = vec!["AACCGGTA".into(), "AACCGCTA".into(), "AAACGGTA".into()];
        assert_eq!(2, Solution::min_mutation(start, end, bank));
    }
}
