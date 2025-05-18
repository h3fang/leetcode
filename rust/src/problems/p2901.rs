pub struct Solution;

use std::collections::HashMap;

fn hash(w: &[u8]) -> u64 {
    let mut ans = 0;
    for (i, b) in w.iter().enumerate() {
        ans |= ((b & 31) as u64) << (i * 5);
    }
    ans
}

fn max_f(
    i: usize,
    w: &str,
    g: i32,
    h: u64,
    groups: &[i32],
    from: &mut [usize],
    m: &mut HashMap<u64, (usize, usize, usize, usize)>,
) -> usize {
    let mut f = 0;
    for k in 0..w.len() {
        let h1 = h | (31 << (k * 5));
        let (max1, j1, max2, j2) = m.get(&h1).cloned().unwrap_or((0, 0, 0, 0));
        if g != groups[j1] {
            if max1 > f {
                f = max1;
                from[i] = j1;
            }
        } else if max2 > f {
            f = max2;
            from[i] = j2;
        }
    }
    f + 1
}

fn update_f(
    i: usize,
    w: &str,
    g: i32,
    h: u64,
    f: usize,
    groups: &[i32],
    m: &mut HashMap<u64, (usize, usize, usize, usize)>,
) {
    for k in 0..w.len() {
        let h1 = h | (31 << (k * 5));
        let (mut max1, mut j1, mut max2, mut j2) = m.get(&h1).cloned().unwrap_or((0, 0, 0, 0));
        if f > max1 {
            if g != groups[j1] {
                max2 = max1;
                j2 = j1;
            }
            max1 = f;
            j1 = i;
        } else if f > max2 && g != groups[j1] {
            max2 = f;
            j2 = i;
        }
        m.insert(h1, (max1, j1, max2, j2));
    }
}

impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = words.len();
        let mut m = HashMap::with_capacity(2048);
        let mut from = vec![0; n];
        let (mut max, mut max_i) = (0, 0);
        for (i, (w, &g)) in words.iter().zip(&groups).enumerate().rev() {
            let h = hash(w.as_bytes());
            let f = max_f(i, w, g, h, &groups, &mut from, &mut m);
            if f > max {
                max = f;
                max_i = i;
            }
            update_f(i, w, g, h, f, &groups, &mut m);
        }

        let mut ans = Vec::with_capacity(max);
        for _ in 0..max {
            ans.push(words[max_i].to_string());
            max_i = from[max_i];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["bab", "dab", "cab"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let groups = vec![1, 2, 2];
        let ans = Solution::get_words_in_longest_subsequence(words, groups);
        let expected: Vec<Vec<String>> = [["bab", "cab"], ["bab", "dab"]]
            .iter()
            .map(|v| v.iter().map(|w| w.to_string()).collect())
            .collect();
        assert!(expected.contains(&ans));
    }

    #[test]
    fn case2() {
        let words = ["a", "b", "c", "d"].iter().map(|w| w.to_string()).collect();
        let groups = vec![1, 2, 3, 4];
        let ans = Solution::get_words_in_longest_subsequence(words, groups);
        let expected: Vec<String> = ["a", "b", "c", "d"].iter().map(|w| w.to_string()).collect();
        assert_eq!(expected, ans);
    }
}
