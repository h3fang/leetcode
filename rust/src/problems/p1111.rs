pub struct Solution;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        seq.bytes()
            .enumerate()
            .map(|(i, b)| ((i + usize::from(b)) % 2) as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn max_depth(ans: &[i32], s: &[u8]) -> i32 {
        let mut balance = [0; 2];
        let mut max = [0; 2];
        for (&id, &byte) in ans.iter().zip(s) {
            let id = id as usize;
            if byte == b'(' {
                balance[id] += 1;
            } else {
                balance[id] -= 1;
            }
            assert!(balance[id] >= 0);
            max[id] = max[id].max(balance[id]);
        }
        assert!(balance.iter().all(|&b| b == 0));
        max[0].max(max[1])
    }

    #[test]
    fn case1() {
        let s = "(()())";
        let ans = Solution::max_depth_after_split(s.into());
        assert_eq!(1, max_depth(&ans, s.as_bytes()));
    }

    #[test]
    fn case2() {
        let s = "()(())()";
        let ans = Solution::max_depth_after_split(s.into());
        assert_eq!(1, max_depth(&ans, s.as_bytes()));
    }
}
