pub struct Solution;

use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    next: HashMap<String, Trie>,
    name: String,
}

fn generate(t: &mut Trie, m: &mut HashMap<String, usize>) {
    if t.next.is_empty() {
        return;
    }
    let mut exprs = Vec::with_capacity(t.next.len());
    for (s, t) in t.next.iter_mut() {
        generate(t, m);
        exprs.push(format!("{}({})", s, t.name));
    }
    exprs.sort_unstable();
    t.name = exprs.join("");

    *m.entry(t.name.clone()).or_default() += 1;
}

fn dfs(t: &Trie, m: &HashMap<String, usize>, path: &mut Vec<String>, ans: &mut Vec<Vec<String>>) {
    if m.get(&t.name).is_some_and(|&v| v > 1) {
        return;
    }
    if !path.is_empty() {
        ans.push(path.clone());
    }

    for (s, t) in &t.next {
        path.push(s.clone());
        dfs(t, m, path, ans);
        path.pop();
    }
}

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let n = paths.len();
        let mut root = Trie::default();
        for p in paths {
            let mut t = &mut root;
            for s in p {
                t = t.next.entry(s).or_default();
            }
        }
        let mut m = HashMap::with_capacity(1024);
        generate(&mut root, &mut m);

        let mut ans = Vec::with_capacity(n);
        let mut path = Vec::with_capacity(n);
        dfs(&root, &m, &mut path, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let paths = [
            vec!["a"],
            vec!["c"],
            vec!["d"],
            vec!["a", "b"],
            vec!["c", "b"],
            vec!["d", "a"],
        ]
        .iter()
        .map(|v| v.iter().map(|p| p.to_string()).collect())
        .collect();
        let mut result = Solution::delete_duplicate_folder(paths);
        result.sort_unstable();

        let mut expected: Vec<Vec<String>> = [vec!["d"], vec!["d", "a"]]
            .iter()
            .map(|v| v.iter().map(|p| p.to_string()).collect())
            .collect();
        expected.sort_unstable();

        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let paths = [
            vec!["a"],
            vec!["c"],
            vec!["a", "b"],
            vec!["c", "b"],
            vec!["a", "b", "x"],
            vec!["a", "b", "x", "y"],
            vec!["w"],
            vec!["w", "y"],
        ]
        .iter()
        .map(|v| v.iter().map(|p| p.to_string()).collect())
        .collect();
        let mut result = Solution::delete_duplicate_folder(paths);
        result.sort_unstable();

        let mut expected: Vec<Vec<String>> = [vec!["c"], vec!["c", "b"], vec!["a"], vec!["a", "b"]]
            .iter()
            .map(|v| v.iter().map(|p| p.to_string()).collect())
            .collect();
        expected.sort_unstable();

        assert_eq!(expected, result);
    }
}
