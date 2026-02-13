pub struct Solution;

use std::collections::HashMap;

fn longest_balanced_1(s: &[u8]) -> i32 {
    let (mut ans, mut cnt) = (1, 1);
    for w in s.windows(2) {
        if w[0] == w[1] {
            cnt += 1;
            ans = ans.max(cnt);
        } else {
            cnt = 1;
        }
    }
    ans
}

fn longest_balanced_2(s: &[u8], ans: &mut i32) {
    fn helper(s: &[u8], ans: &mut i32, a: u8, b: u8, pre: &mut HashMap<i32, i32>) {
        for w in s.split(|&x| x != a && x != b) {
            if w.len() as i32 <= *ans {
                continue;
            }

            pre.clear();
            pre.insert(0, -1);
            let mut sum = 0;
            for (i, &x) in w.iter().enumerate() {
                sum += if x == a { 1 } else { -1 };
                if let Some(j) = pre.get(&sum) {
                    *ans = (*ans).max(i as i32 - j);
                } else {
                    pre.insert(sum, i as i32);
                }
            }
        }
    }

    let mut m = HashMap::with_capacity(s.len() * 2);
    helper(s, ans, b'a', b'b', &mut m);
    helper(s, ans, b'a', b'c', &mut m);
    helper(s, ans, b'b', b'c', &mut m);
}

fn longest_balanced_3(s: &[u8]) -> i32 {
    let mut cnt = [0; 3];
    let mut pre = HashMap::with_capacity(s.len() * 2);
    pre.insert((0, 0), -1);
    let mut ans = 0;
    for (i, &b) in s.iter().enumerate() {
        cnt[(b - b'a') as usize] += 1;
        let p = (cnt[1] - cnt[0], cnt[2] - cnt[0]);
        if let Some(j) = pre.get(&p) {
            ans = ans.max(i as i32 - j);
        } else {
            pre.insert(p, i as i32);
        }
    }
    ans
}

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = longest_balanced_1(s).max(longest_balanced_3(s));
        longest_balanced_2(s, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::longest_balanced("abbac".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::longest_balanced("aabcc".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::longest_balanced("aba".to_string()));
    }
}
