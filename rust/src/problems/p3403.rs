pub struct Solution;

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }
        let n = word.len();
        let s = word.as_bytes();
        let len = n - num_friends as usize + 1;
        let (mut i, mut j) = (0, 1);
        while j < n {
            let mut k = 0;
            while j + k < n && s[i + k] == s[j + k] {
                k += 1;
            }
            if j + k < n && s[i + k] < s[j + k] {
                (i, j) = (j, (j + 1).max(i + k + 1));
            } else {
                j += k + 1;
            }
        }
        word[i..(i + len).min(n)].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("dbc", Solution::answer_string("dbca".to_string(), 2));
    }

    #[test]
    fn case2() {
        assert_eq!("g", Solution::answer_string("gggg".to_string(), 4));
    }
}
