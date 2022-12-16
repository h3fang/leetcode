use std::collections::{HashMap, HashSet};

pub struct Solution;

struct DisjointSetUnion {
    parents: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] != x {
            self.parents[x] = self.find(self.parents[x]);
        }
        self.parents[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }
        match self.rank[px].cmp(&self.rank[py]) {
            std::cmp::Ordering::Less => self.parents[px] = py,
            std::cmp::Ordering::Greater => self.parents[py] = px,
            std::cmp::Ordering::Equal => {
                self.parents[py] = px;
                self.rank[px] += 1;
            }
        }
    }
}

impl Solution {
    pub fn accounts_merge_bf(mut accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        loop {
            let mut done = true;
            for i in 0..accounts.len() {
                if accounts[i].len() == 1 {
                    continue;
                }
                let mut emails = accounts[i].iter().skip(1).cloned().collect::<HashSet<_>>();
                for j in i + 1..accounts.len() {
                    if accounts[i][0] == accounts[j][0]
                        && accounts[j].iter().skip(1).any(|e| emails.contains(e))
                    {
                        done = false;
                        for e in &accounts[j][1..] {
                            emails.insert(e.clone());
                        }
                        accounts[j] = vec![j.to_string()];
                    }
                }
                accounts[i].resize(1, "".to_string());
                accounts[i].extend(emails.into_iter());
            }
            if done {
                break;
            }
        }

        accounts
            .into_iter()
            .filter(|acc| acc.len() > 1)
            .map(|mut acc| {
                let name = acc.remove(0);
                acc.sort_unstable();
                acc.insert(0, name);
                acc
            })
            .collect()
    }

    pub fn accounts_merge_dfs(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut graph: HashMap<&String, Vec<&String>> = HashMap::new();
        let mut email_names = HashMap::new();
        for acc in &accounts {
            for e in acc.iter().skip(1) {
                graph.entry(&acc[1]).or_default().push(e);
                graph.entry(e).or_default().push(&acc[1]);
                email_names.insert(e, &acc[0]);
            }
        }

        let mut seen = HashSet::new();
        let mut result = Vec::new();

        for email in graph.keys() {
            if !seen.contains(email) {
                seen.insert(email);
                let mut stack = vec![email];
                let mut emails = Vec::new();
                while let Some(e) = stack.pop() {
                    emails.push(e.to_string());
                    for next in graph.get(e).unwrap() {
                        if !seen.contains(next) {
                            seen.insert(next);
                            stack.push(next);
                        }
                    }
                }
                emails.sort_unstable();
                emails.insert(0, email_names.get(email).unwrap().to_string());
                result.push(emails);
            }
        }

        result
    }

    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut dsu = DisjointSetUnion::new(10001);
        let mut email_names = HashMap::new();
        let mut email_ids = HashMap::new();
        let mut i = 0;
        for acc in &accounts {
            for e in acc.iter().skip(1) {
                email_names.insert(e, &acc[0]);
                if !email_ids.contains_key(e) {
                    email_ids.insert(e, i);
                    i += 1;
                }
                dsu.union(
                    *email_ids.get(&e).unwrap(),
                    *email_ids.get(&acc[1]).unwrap(),
                );
            }
        }

        let mut result: HashMap<usize, Vec<String>> = HashMap::new();

        for (email, id) in email_ids {
            result
                .entry(dsu.find(id))
                .or_default()
                .push(email.to_string());
        }

        result
            .into_values()
            .map(|mut emails| {
                emails.sort_unstable();
                let name = email_names.get(&emails[0]).unwrap();
                emails.insert(0, name.to_string());
                emails
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(accounts: Vec<Vec<&str>>, expected: Vec<Vec<&str>>) {
        let accounts = accounts
            .iter()
            .map(|a| a.iter().map(|s| s.to_string()).collect())
            .collect::<Vec<_>>();
        let mut expected = expected
            .iter()
            .map(|a| a.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::accounts_merge(accounts.clone());
        result.sort_unstable();
        assert_eq!(expected, result);

        let mut result = Solution::accounts_merge_dfs(accounts.clone());
        result.sort_unstable();
        assert_eq!(expected, result);

        let mut result = Solution::accounts_merge_bf(accounts);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case1() {
        let accounts = vec![
            vec!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
            vec!["John", "johnsmith@mail.com", "john00@mail.com"],
            vec!["Mary", "mary@mail.com"],
            vec!["John", "johnnybravo@mail.com"],
        ];
        let expected = vec![
            vec![
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com",
            ],
            vec!["Mary", "mary@mail.com"],
            vec!["John", "johnnybravo@mail.com"],
        ];
        helper(accounts, expected);
    }

    #[test]
    fn case2() {
        let accounts = vec![
            vec!["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe1@m.co"],
            vec!["Kevin", "Kevin3@m.co", "Kevin5@m.co", "Kevin0@m.co"],
            vec!["Ethan", "Ethan5@m.co", "Ethan4@m.co", "Ethan0@m.co"],
            vec!["Hanzo", "Hanzo3@m.co", "Hanzo1@m.co", "Hanzo0@m.co"],
            vec!["Fern", "Fern5@m.co", "Fern1@m.co", "Fern0@m.co"],
        ];
        let expected = vec![
            vec!["Ethan", "Ethan0@m.co", "Ethan4@m.co", "Ethan5@m.co"],
            vec!["Gabe", "Gabe0@m.co", "Gabe1@m.co", "Gabe3@m.co"],
            vec!["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co", "Hanzo3@m.co"],
            vec!["Kevin", "Kevin0@m.co", "Kevin3@m.co", "Kevin5@m.co"],
            vec!["Fern", "Fern0@m.co", "Fern1@m.co", "Fern5@m.co"],
        ];
        helper(accounts, expected);
    }

    #[test]
    fn case3() {
        let accounts = vec![
            vec!["David", "David0@m.co", "David4@m.co", "David3@m.co"],
            vec!["David", "David5@m.co", "David5@m.co", "David0@m.co"],
            vec!["David", "David1@m.co", "David4@m.co", "David0@m.co"],
            vec!["David", "David0@m.co", "David1@m.co", "David3@m.co"],
            vec!["David", "David4@m.co", "David1@m.co", "David3@m.co"],
        ];
        let expected = vec![vec![
            "David",
            "David0@m.co",
            "David1@m.co",
            "David3@m.co",
            "David4@m.co",
            "David5@m.co",
        ]];
        helper(accounts, expected);
    }

    #[test]
    fn case4() {
        let accounts = vec![
            vec!["David", "David0@m.co", "David1@m.co"],
            vec!["David", "David3@m.co", "David4@m.co"],
            vec!["David", "David4@m.co", "David5@m.co"],
            vec!["David", "David2@m.co", "David3@m.co"],
            vec!["David", "David1@m.co", "David2@m.co"],
        ];
        let expected = vec![vec![
            "David",
            "David0@m.co",
            "David1@m.co",
            "David2@m.co",
            "David3@m.co",
            "David4@m.co",
            "David5@m.co",
        ]];
        helper(accounts, expected);
    }

    #[test]
    fn case5() {
        let accounts = vec![
            vec!["Alex", "Alex5@m.co", "Alex4@m.co", "Alex0@m.co"],
            vec!["Ethan", "Ethan3@m.co", "Ethan3@m.co", "Ethan0@m.co"],
            vec!["Kevin", "Kevin4@m.co", "Kevin2@m.co", "Kevin2@m.co"],
            vec!["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe2@m.co"],
            vec!["Gabe", "Gabe3@m.co", "Gabe4@m.co", "Gabe2@m.co"],
        ];
        let expected = vec![
            vec!["Alex", "Alex0@m.co", "Alex4@m.co", "Alex5@m.co"],
            vec!["Ethan", "Ethan0@m.co", "Ethan3@m.co"],
            vec![
                "Gabe",
                "Gabe0@m.co",
                "Gabe2@m.co",
                "Gabe3@m.co",
                "Gabe4@m.co",
            ],
            vec!["Kevin", "Kevin2@m.co", "Kevin4@m.co"],
        ];
        helper(accounts, expected);
    }
}
