pub struct Solution;

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let mut ans = i32::MIN;
        for a in b'0'..=b'4' {
            let x = (a - b'0') as usize;
            for b in b'0'..=b'4' {
                let y = (b - b'0') as usize;
                if x == y {
                    continue;
                }
                let mut cur = [0; 5];
                let mut pre = [0; 5];
                let mut min = [[i32::MAX / 2; 2]; 2];
                let mut l = 0;
                for (i, &c) in s.iter().enumerate() {
                    cur[(c - b'0') as usize] += 1;
                    let r = i + 1;
                    while r - l >= k && cur[x] > pre[x] && cur[y] > pre[y] {
                        let p = pre[x] & 1;
                        let q = pre[y] & 1;
                        min[p][q] = min[p][q].min(pre[x] as i32 - pre[y] as i32);
                        pre[(s[l] - b'0') as usize] += 1;
                        l += 1;
                    }
                    if r >= k {
                        let r = cur[x] as i32 - cur[y] as i32 - min[(cur[x] & 1) ^ 1][cur[y] & 1];
                        ans = ans.max(r);
                    }
                }
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
        assert_eq!(-1, Solution::max_difference("12233".to_string(), 4));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::max_difference("1122211".to_string(), 3));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::max_difference("110".to_string(), 3));
    }
}
