pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        let mut m = HashMap::new();
        names
            .into_iter()
            .map(|n| {
                if let Some(i) = m.get(&n) {
                    let mut i = i + 1;
                    let mut r = format!("{n}({i})");
                    while m.contains_key(&r) {
                        i += 1;
                        r = format!("{n}({i})");
                    }
                    m.insert(n.to_string(), i);
                    m.insert(r.to_string(), 0);
                    r
                } else {
                    m.insert(n.to_string(), 0);
                    n
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let names = ["pes", "fifa", "gta", "pes(2019)"]
            .iter()
            .map(|n| n.to_string())
            .collect();
        let expected = ["pes", "fifa", "gta", "pes(2019)"]
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::get_folder_names(names));
    }

    #[test]
    fn case2() {
        let names = ["gta", "gta(1)", "gta", "avalon"]
            .iter()
            .map(|n| n.to_string())
            .collect();
        let expected = ["gta", "gta(1)", "gta(2)", "avalon"]
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::get_folder_names(names));
    }

    #[test]
    fn case3() {
        let names = ["kaido", "kaido(1)", "kaido", "kaido(1)"]
            .iter()
            .map(|n| n.to_string())
            .collect();
        let expected = ["kaido", "kaido(1)", "kaido(2)", "kaido(1)(1)"]
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::get_folder_names(names));
    }
}
