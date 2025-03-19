pub struct Solution;

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let masks = arr
            .iter()
            .filter_map(|s| {
                let mut mask = 0u32;
                for b in s.as_bytes() {
                    let i = (b - b'a') as u32;
                    if mask & (1 << i) > 0 {
                        mask = 0;
                        break;
                    } else {
                        mask |= 1 << i;
                    }
                }
                if mask > 0 { Some(mask) } else { None }
            })
            .collect::<Vec<_>>();

        fn dfs(masks: &[u32], i: usize, m: u32, result: &mut i32) {
            if i == masks.len() {
                *result = (*result).max(m.count_ones() as i32);
                return;
            }
            if masks[i] & m == 0 {
                dfs(masks, i + 1, masks[i] | m, result);
            }
            dfs(masks, i + 1, m, result);
        }

        let mut result = 0;
        dfs(&masks, 0, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let arr = ["un", "iq", "ue"].iter().map(|s| s.to_string()).collect();
        assert_eq!(4, Solution::max_length(arr));
    }

    #[test]
    fn case2() {
        let arr = ["cha", "r", "act", "ers"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(6, Solution::max_length(arr));
    }

    #[test]
    fn case3() {
        let arr = ["abcdefghijklmnopqrstuvwxyz"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(26, Solution::max_length(arr));
    }
}
