pub struct Solution;

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let mut pos = [(-1, -1); 26];

        for (i, &b) in bytes.iter().enumerate() {
            let i = i as i32;
            let j = (b - b'a') as usize;
            if pos[j].0 == -1 {
                pos[j] = (i, i);
            } else {
                pos[j].1 = i;
            }
        }

        let mut valid: Vec<(i32, i32)> = Vec::with_capacity(26);

        for &(mut l, mut r) in &pos {
            if l == -1 {
                continue;
            }
            let mut nl = l;
            let mut nr = l;

            while nl >= l || nr <= r {
                let i = if nl >= l { nl } else { nr };

                let c = bytes[i as usize];
                let (l_t, r_t) = pos[(c - b'a') as usize];

                l = l_t.min(l);
                r = r_t.max(r);

                nl -= i32::from(i == nl);
                nr += i32::from(i == nr);
            }

            valid.push((l, r));
        }

        valid.sort_by_key(|a| a.1);

        let mut ans: Vec<String> = Vec::with_capacity(valid.len());
        let mut end = -1;

        for (left, right) in valid {
            if left > end {
                let sub = bytes[left as usize..(right as usize + 1)].to_vec();
                let sub = unsafe { String::from_utf8_unchecked(sub) };
                ans.push(sub);
                end = right;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut expected = ["e", "f", "ccc"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        expected.sort_unstable();

        let mut result = Solution::max_num_of_substrings("adefaddaccc".to_string());
        result.sort_unstable();

        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let mut expected = ["d", "bb", "cc"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        expected.sort_unstable();

        let mut result = Solution::max_num_of_substrings("abbaccd".to_string());
        result.sort_unstable();

        assert_eq!(expected, result);
    }
}
