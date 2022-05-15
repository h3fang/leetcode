pub struct Solution;

impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        fn dominate(c1: &[i8], c2: &[i8]) -> bool {
            c1.iter().zip(c2).all(|(a, b)| a >= b)
        }

        let mut t = 0u32;
        for &b in target.as_bytes() {
            let i = (b - b'a') as usize;
            t |= 1 << i;
        }

        let mut counts: Vec<[i8; 26]> = vec![];
        'outter: for s in stickers {
            let mut count = [0; 26];
            s.as_bytes().iter().for_each(|c| {
                let i = (c - b'a') as usize;
                if t & (1 << i) > 0 {
                    count[i] += 1;
                }
            });
            for c in &counts {
                if dominate(c, &count) {
                    continue 'outter;
                }
            }
            counts = counts
                .into_iter()
                .filter(|c| !dominate(&count, c))
                .collect();
            counts.push(count);
        }

        fn dfs(dp: &mut [i32], m: i32, counts: &[[i8; 26]], target: &[u8], mask: u16) -> i32 {
            if dp[mask as usize] >= 0 {
                return dp[mask as usize];
            }
            let mut result = m + 1;
            for c in counts {
                let mut c = *c;
                let mut remaining = mask;
                for (i, b) in target.iter().enumerate() {
                    let j = (*b - b'a') as usize;
                    if mask & (1 << i) > 0 && c[j] > 0 {
                        c[j] -= 1;
                        remaining ^= 1 << i;
                    }
                }
                if remaining < mask {
                    result = result.min(dfs(dp, m, counts, target, remaining) + 1);
                }
            }
            dp[mask as usize] = result;
            result
        }

        let m = target.len();
        let mut dp = vec![-1; 1 << m];
        dp[0] = 0;
        let r = dfs(&mut dp, m as i32, &counts, target.as_bytes(), (1 << m) - 1);
        if r <= m as i32 {
            r
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let stickers = ["with", "example", "science"];
        let stickers = stickers.iter().map(|s| s.to_string()).collect();
        let target = "thehat".to_string();
        assert_eq!(3, Solution::min_stickers(stickers, target));
    }

    #[test]
    fn case2() {
        let stickers = ["notice", "possible"];
        let stickers = stickers.iter().map(|s| s.to_string()).collect();
        let target = "basicbasic".to_string();
        assert_eq!(-1, Solution::min_stickers(stickers, target));
    }

    #[test]
    fn case3() {
        let stickers = [
            "control", "heart", "interest", "stream", "sentence", "soil", "wonder", "them",
            "month", "slip", "table", "miss", "boat", "speak", "figure", "no", "perhaps", "twenty",
            "throw", "rich", "capital", "save", "method", "store", "meant", "life", "oil",
            "string", "song", "food", "am", "who", "fat", "if", "put", "path", "come", "grow",
            "box", "great", "word", "object", "stead", "common", "fresh", "the", "operate",
            "where", "road", "mean",
        ];
        let stickers = stickers.iter().map(|s| s.to_string()).collect();
        let target = "stoodcrease".to_string();
        assert_eq!(3, Solution::min_stickers(stickers, target));
    }
}
