use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        words.sort_unstable_by_key(|w| w.as_bytes().len());
        let mut dp: HashMap<&str, usize> = HashMap::new();
        let mut result = 0;
        for w in &words {
            let n = w.len();
            for i in 0..n {
                let mut w1 = String::with_capacity(n - 1);
                w1 += &w[..i];
                w1 += &w[i + 1..];
                let len = *dp.get(&w1.as_str()).unwrap_or(&0) + 1;
                let e = dp.entry(w).or_default();
                *e = (*e).max(len);
            }
            result = result.max(*dp.get(&w.as_str()).unwrap());
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["a", "b", "ba", "bca", "bda", "bdca"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!(4, Solution::longest_str_chain(words));
    }

    #[test]
    fn case2() {
        let words = ["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!(5, Solution::longest_str_chain(words));
    }

    #[test]
    fn case3() {
        let words = ["abcd", "dbqca"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!(1, Solution::longest_str_chain(words));
    }
}
