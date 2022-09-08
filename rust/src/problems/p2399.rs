pub struct Solution;

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let s = s.as_bytes();
        let mut mask = 0;
        for (i, &c) in s.iter().enumerate() {
            let j = (c - b'a') as usize;
            if mask & (1 << j) > 0 {
                continue;
            }
            mask |= 1 << j;
            let i2 = i + distance[j] as usize + 1;
            if i2 >= s.len() || s[i2] != c {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "abaccb".to_string();
        let distance = vec![
            1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(true, Solution::check_distances(s, distance));
    }
}
