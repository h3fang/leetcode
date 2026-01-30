pub struct Solution;

struct Trie {
    next: [Option<Box<Trie>>; 26],
    id: i32,
}

impl Default for Trie {
    fn default() -> Self {
        Self {
            next: Default::default(),
            id: -1,
        }
    }
}

impl Trie {
    fn insert(&mut self, word: &str, idx: &mut i32) -> i32 {
        let mut t = self;
        for &b in word.as_bytes() {
            let i = (b - b'a') as usize;
            t = t.next[i].get_or_insert_default();
        }
        if t.id == -1 {
            t.id = *idx;
            *idx += 1;
        }
        t.id
    }
}

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        let (n, m) = (source.len(), original.len());

        let mut g = vec![vec![i32::MAX / 2; m * 2]; m * 2];
        g.iter_mut().enumerate().for_each(|(i, v)| v[i] = 0);

        let mut idx = 0;
        let mut trie = Trie::default();
        for ((a, b), &c) in original.iter().zip(&changed).zip(&cost) {
            let x = trie.insert(a, &mut idx);
            let y = trie.insert(b, &mut idx);
            g[x as usize][y as usize] = g[x as usize][y as usize].min(c);
        }

        let p = idx as usize;

        for k in 0..p {
            for i in 0..p {
                if g[i][k] == i32::MAX / 2 {
                    continue;
                }
                for j in 0..p {
                    g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
                }
            }
        }

        let (s, t) = (source.as_bytes(), target.as_bytes());
        let mut f = vec![0; n + 1];

        for i in (0..n).rev() {
            f[i] = if s[i] == t[i] { f[i + 1] } else { i64::MAX / 2 };

            let (mut u, mut v) = (&trie, &trie);
            for (j, (s, t)) in s.iter().zip(t).enumerate().skip(i) {
                match (&u.next[(s - b'a') as usize], &v.next[(t - b'a') as usize]) {
                    (Some(x), Some(y)) => {
                        u = x;
                        v = y;
                    }
                    _ => break,
                }

                if u.id == -1 || v.id == -1 {
                    continue;
                }

                let d = g[u.id as usize][v.id as usize];
                if d < i32::MAX / 2 {
                    f[i] = f[i].min(d as i64 + f[j + 1]);
                }
            }
        }
        if f[0] < i64::MAX / 2 { f[0] } else { -1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let source = "abcd".to_string();
        let target = "acbe".to_string();
        let original = ["a", "b", "c", "c", "e", "d"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let changed = ["b", "c", "b", "e", "b", "e"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let cost = vec![2, 5, 5, 1, 2, 20];
        assert_eq!(
            28,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn case2() {
        let source = "abcdefgh".to_string();
        let target = "acdeeghh".to_string();
        let original = ["bcd", "fgh", "thh"]
            .iter()
            .map(|x| x.to_string())
            .map(|x| x.to_string())
            .collect();
        let changed = ["cde", "thh", "ghh"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let cost = vec![1, 3, 5];
        assert_eq!(
            9,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn case3() {
        let source = "abcdefgh".to_string();
        let target = "addddddd".to_string();
        let original = ["bcd", "defgh"].iter().map(|x| x.to_string()).collect();
        let changed = ["ddd", "ddddd"].iter().map(|x| x.to_string()).collect();
        let cost = vec![100, 1578];
        assert_eq!(
            -1,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }
}
