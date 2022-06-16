use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut map: HashMap<&str, u32> = HashMap::new();
        for idea in &ideas {
            *map.entry(&idea[1..]).or_default() |= 1 << (idea.as_bytes()[0] - b'a');
        }
        let mut f = [[0; 26]; 26];
        for idea in &ideas {
            let mask = *map.get(&idea[1..]).unwrap();
            let w = (idea.as_bytes()[0] - b'a') as usize;
            for c in b'a'..=b'z' {
                if mask & (1 << (c - b'a')) == 0 {
                    f[w][(c - b'a') as usize] += 1;
                }
            }
        }
        let mut result = 0;
        for idea in &ideas {
            let mask = *map.get(&idea[1..]).unwrap();
            let w = (idea.as_bytes()[0] - b'a') as usize;
            for c in b'a'..=b'z' {
                if mask & (1 << (c - b'a')) == 0 {
                    result += f[(c - b'a') as usize][w];
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
        let ideas = ["coffee", "donuts", "time", "toffee"];
        let ideas = ideas.iter().map(|i| i.to_string()).collect();
        assert_eq!(6, Solution::distinct_names(ideas));
    }

    #[test]
    fn case2() {
        let ideas = ["lack", "back"];
        let ideas = ideas.iter().map(|i| i.to_string()).collect();
        assert_eq!(0, Solution::distinct_names(ideas));
    }
}
