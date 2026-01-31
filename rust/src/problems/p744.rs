pub struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let i = letters.partition_point(|&c| c <= target);
        if i == letters.len() {
            letters[0]
        } else {
            letters[i]
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
