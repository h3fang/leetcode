pub struct Solution;

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let p = pattern.as_bytes();
        queries
            .iter()
            .map(|q| {
                let q = q.as_bytes();
                let mut i = 0;
                for &c in q {
                    if i < p.len() && p[i] == c {
                        i += 1;
                    } else if c.is_ascii_uppercase() {
                        return false;
                    }
                }
                i >= p.len()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let queries = [
            "FooBar",
            "FooBarTest",
            "FootBall",
            "FrameBuffer",
            "ForceFeedBack",
        ]
        .iter()
        .map(|q| q.to_string())
        .collect();
        let pattern = "FB".to_string();
        assert_eq!(
            vec![true, false, true, true, false],
            Solution::camel_match(queries, pattern)
        );
    }
}
