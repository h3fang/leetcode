pub struct Solution;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        let n = s.len();
        if n != goal.len() {
            return false;
        }
        let mut first = usize::MAX;
        let mut second = usize::MAX;
        let s = s.as_bytes();
        let goal = goal.as_bytes();
        for (i, (a, b)) in s.iter().zip(goal.iter()).enumerate() {
            if a != b {
                if first == usize::MAX {
                    first = i;
                } else if second == usize::MAX {
                    second = i;

                    if s[first] != goal[second] || s[second] != goal[first] {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        if first != usize::MAX && second == usize::MAX {
            false
        } else if first == usize::MAX {
            if n > 26 {
                true
            } else {
                let mut count = [0; 26];
                for c in s {
                    count[(c - b'a') as usize] += 1;
                    if count[(c - b'a') as usize] > 1 {
                        return true;
                    }
                }
                false
            }
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "ab".to_string();
        let goal = "ba".to_string();
        assert_eq!(true, Solution::buddy_strings(s, goal));
    }

    #[test]
    fn case2() {
        let s = "ab".to_string();
        let goal = "ab".to_string();
        assert_eq!(false, Solution::buddy_strings(s, goal));
    }

    #[test]
    fn case3() {
        let s = "aa".to_string();
        let goal = "aa".to_string();
        assert_eq!(true, Solution::buddy_strings(s, goal));
    }

    #[test]
    fn case4() {
        let s = "aaaaaaabc".to_string();
        let goal = "aaaaaaacb".to_string();
        assert_eq!(true, Solution::buddy_strings(s, goal));
    }
}
