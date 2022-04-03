pub struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let n = letters.len() as i32;
        let mut left = 0;
        let mut right = n - 1;
        let mut idx = n;
        while left <= right {
            let mid = (left + right) / 2;
            match letters[mid as usize].cmp(&target) {
                std::cmp::Ordering::Greater => {
                    idx = mid;
                    right = mid - 1;
                }
                _ => {
                    left = mid + 1;
                }
            }
        }
        if idx == n {
            letters[0]
        } else {
            letters[idx as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let letters = ["c", "f", "j"];
        let letters = letters.iter().map(|e| e.chars().next().unwrap()).collect();
        assert_eq!('c', Solution::next_greatest_letter(letters, 'a'));
    }

    #[test]
    fn case2() {
        let letters = ["c", "f", "j"];
        let letters = letters.iter().map(|e| e.chars().next().unwrap()).collect();
        assert_eq!('f', Solution::next_greatest_letter(letters, 'c'));
    }

    #[test]
    fn case3() {
        let letters = ["c", "f", "j"];
        let letters = letters.iter().map(|e| e.chars().next().unwrap()).collect();
        assert_eq!('f', Solution::next_greatest_letter(letters, 'd'));
    }
}
