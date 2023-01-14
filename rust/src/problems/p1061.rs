pub struct Solution;

struct Dsu {
    parent: [usize; 26],
}

impl Dsu {
    fn new() -> Self {
        let mut parent = [0; 26];
        for (i, p) in parent.iter_mut().enumerate() {
            *p = i;
        }
        Self { parent }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }
        if px < py {
            self.parent[py] = px;
        } else {
            self.parent[px] = py;
        }
    }
}

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, mut base_str: String) -> String {
        let mut dsu = Dsu::new();
        for (&a, &b) in s1.as_bytes().iter().zip(s2.as_bytes()) {
            let x = (a - b'a') as usize;
            let y = (b - b'a') as usize;
            dsu.union(x, y);
        }
        let base = unsafe { base_str.as_bytes_mut() };
        for b in base {
            let x = (*b - b'a') as usize;
            *b = dsu.find(x) as u8 + b'a';
        }
        base_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "makkek",
            Solution::smallest_equivalent_string(
                "parker".to_string(),
                "morris".to_string(),
                "parser".to_string()
            )
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "hdld",
            Solution::smallest_equivalent_string(
                "hello".to_string(),
                "world".to_string(),
                "hold".to_string()
            )
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "aauaaaaada",
            Solution::smallest_equivalent_string(
                "leetcode".to_string(),
                "programs".to_string(),
                "sourcecode".to_string()
            )
        );
    }
}
