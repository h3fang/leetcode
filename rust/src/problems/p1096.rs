pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        fn expand(s: &str, mut i: usize) -> (usize, HashSet<String>) {
            let mut result: HashSet<String> = HashSet::new();
            while i < s.len() {
                match s.as_bytes()[i] {
                    b'{' => {
                        let (j, suffix) = expand(s, i + 1);
                        if result.is_empty() {
                            result = suffix;
                        } else {
                            result = result
                                .iter()
                                .flat_map(|e| suffix.iter().map(|s| e.to_string() + s.as_str()))
                                .collect();
                        }
                        i = j;
                    }
                    b'}' => return (i + 1, result),
                    b',' => {
                        let (j, suffix) = expand(s, i + 1);
                        result = result.union(&suffix).cloned().collect();
                        return (j, result);
                    }
                    x => {
                        if result.is_empty() {
                            result.insert((x as char).to_string());
                        } else {
                            result = result
                                .into_iter()
                                .map(|mut e| {
                                    e.push(x as char);
                                    e
                                })
                                .collect();
                        }
                        i += 1;
                    }
                }
            }
            (i, result)
        }
        let (_, result) = expand(&expression, 0);
        let mut result = result.into_iter().collect::<Vec<_>>();
        result.sort_unstable();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = ["ac", "ad", "ae", "bc", "bd", "be"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            result,
            Solution::brace_expansion_ii("{a,b}{c,{d,e}}".to_string())
        );
    }

    #[test]
    fn case2() {
        let result = ["a", "ab", "ac", "z"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            result,
            Solution::brace_expansion_ii("{{a,z},a{b,c},{ab,z}}".to_string())
        );
    }
}
