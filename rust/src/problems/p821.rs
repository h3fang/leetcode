pub struct Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut pos = vec![];
        for (i, e) in s.char_indices() {
            if e == c {
                pos.push(i as i32);
            }
        }

        let mut result = Vec::with_capacity(s.len());
        let mut j = 0;
        for i in 0..s.len() as i32 {
            let k = if j > 0 {
                (pos[j] - i).abs().min((pos[j - 1] - i as i32).abs())
            } else {
                (pos[j] - i as i32).abs()
            };
            result.push(k);
            if i as i32 == pos[j] && j + 1 < pos.len() {
                j += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0],
            Solution::shortest_to_char("loveleetcode".into(), 'e')
        );
    }
}
