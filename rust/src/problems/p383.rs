pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }
        let mut count = [0; 26];
        for c in magazine.as_bytes() {
            count[(c - b'a') as usize] += 1;
        }

        for c in ransom_note.as_bytes() {
            let i = (c - b'a') as usize;
            count[i] -= 1;
            if count[i] < 0 {
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
        assert_eq!(false, Solution::can_construct("a".into(), "b".into()))
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::can_construct("aa".into(), "ab".into()))
    }

    #[test]
    fn case3() {
        assert_eq!(true, Solution::can_construct("aa".into(), "aab".into()))
    }
}
