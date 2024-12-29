pub struct Solution;

impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let n = votes[0].len();
        let mut f = vec![(vec![0; n], 0); 26];
        for v in votes {
            for (i, &b) in v.as_bytes().iter().enumerate() {
                let j = (b - b'A') as usize;
                f[j].1 = b;
                f[j].0[i] -= 1;
            }
        }
        f.sort_unstable();
        f.into_iter()
            .filter_map(|(v, b)| {
                if v.iter().any(|e| *e < 0) {
                    Some(b as char)
                } else {
                    None
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
        let votes = ["ABC", "ACB", "ABC", "ACB", "ACB"]
            .iter()
            .map(|v| v.to_string())
            .collect();
        assert_eq!("ACB", Solution::rank_teams(votes));
    }

    #[test]
    fn case2() {
        let votes = ["WXYZ", "XYZW"].iter().map(|v| v.to_string()).collect();
        assert_eq!("XWYZ", Solution::rank_teams(votes));
    }
}
