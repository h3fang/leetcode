pub struct Solution;

impl Solution {
    pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
        let mut map = vec![[false; 256]; 256];
        for e in mappings {
            map[e[0] as usize][e[1] as usize] = true;
        }

        let s = s.as_bytes();
        let sub = sub.as_bytes();
        let m = s.len();
        let n = sub.len();
        for j in 0..m - n + 1 {
            let mut valid = true;
            for (a, b) in s[j..j + n].iter().zip(sub) {
                if a == b || map[*b as usize][*a as usize] {
                    continue;
                }
                valid = false;
                break;
            }
            if valid {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "fool3e7bar".into();
        let sub = "leet".into();
        let mappings = [["e", "3"], ["t", "7"], ["t", "8"]];
        let mappings = mappings
            .iter()
            .map(|m| m.iter().map(|e| e.as_bytes()[0] as char).collect())
            .collect();
        assert_eq!(true, Solution::match_replacement(s, sub, mappings));
    }
}
