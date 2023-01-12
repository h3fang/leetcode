pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let m = knowledge
            .iter()
            .map(|k| (k[0].as_str(), k[1].as_str()))
            .collect::<HashMap<_, _>>();
        let mut result = String::with_capacity(s.len());
        let mut key = false;
        let mut start = 0;
        for (i, c) in s.char_indices() {
            match c {
                '(' => {
                    key = true;
                    start = i + 1;
                }
                ')' => {
                    let k = &s[start..i];
                    if let Some(v) = m.get(k) {
                        result.push_str(v);
                    } else {
                        result.push('?');
                    }
                    key = false;
                }
                x => {
                    if !key {
                        result.push(x);
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "(name)is(age)yearsold".to_string();
        let knowledge = [["name", "bob"], ["age", "two"]]
            .iter()
            .map(|kv| kv.iter().map(|e| e.to_string()).collect())
            .collect();
        assert_eq!("bobistwoyearsold", Solution::evaluate(s, knowledge));
    }

    #[test]
    fn case2() {
        let s = "hi(name)".to_string();
        let knowledge = [["a", "b"]]
            .iter()
            .map(|kv| kv.iter().map(|e| e.to_string()).collect())
            .collect();
        assert_eq!("hi?", Solution::evaluate(s, knowledge));
    }

    #[test]
    fn case3() {
        let s = "(a)(a)(a)aaa".to_string();
        let knowledge = [["a", "yes"]]
            .iter()
            .map(|kv| kv.iter().map(|e| e.to_string()).collect())
            .collect();
        assert_eq!("yesyesyesaaa", Solution::evaluate(s, knowledge));
    }
}
