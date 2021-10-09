struct Trie {
    next: Vec<Option<Box<Trie>>>,
    word: Option<String>,
}

impl Trie {
    fn new() -> Self {
        Self {
            next: (0..256).map(|_| None).collect(),
            word: None,
        }
    }

    fn from_words(words: Vec<String>) -> Self {
        let mut t = Self::new();
        for w in words {
            t.insert(w);
        }
        t
    }

    fn insert(&mut self, word: String) {
        let bytes = word.as_bytes();
        self.insert_impl(bytes, word.to_string());
    }

    fn insert_impl(&mut self, chars: &[u8], word: String) {
        if chars.is_empty() {
            self.word = Some(word);
            return;
        }

        let c = chars[0];
        match self.next.get_mut(c as usize).unwrap() {
            Some(t) => {
                t.insert_impl(&chars[1..], word);
            }
            None => {
                let mut t = Self::new();
                t.insert_impl(&chars[1..], word);
                self.next[c as usize] = Some(Box::new(t));
            }
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        fn bt(
            board: &mut Vec<Vec<char>>,
            i: i32,
            j: i32,
            height: i32,
            width: i32,
            trie: &mut Trie,
            result: &mut Vec<String>,
        ) {
            if i < 0 || j < 0 || i >= height || j >= width {
                return;
            }

            let c = board[i as usize][j as usize];

            if let Some(t) = trie.next.get_mut(c as usize).unwrap() {
                if let Some(word) = t.word.take() {
                    result.push(word);
                }
                board[i as usize][j as usize] = '{';
                bt(board, i - 1, j, height, width, t, result);
                bt(board, i + 1, j, height, width, t, result);
                bt(board, i, j - 1, height, width, t, result);
                bt(board, i, j + 1, height, width, t, result);
                board[i as usize][j as usize] = c;
            }
        }

        let mut trie = Trie::from_words(words);

        let m = board.len();
        let n = board[0].len();
        let mut board = board;

        let mut result = Vec::new();
        for i in 0..m {
            for j in 0..n {
                bt(
                    &mut board,
                    i as i32,
                    j as i32,
                    m as i32,
                    n as i32,
                    &mut trie,
                    &mut result,
                );
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
        let board = [
            ["o", "a", "a", "n"],
            ["e", "t", "a", "e"],
            ["i", "h", "k", "r"],
            ["i", "f", "l", "v"],
        ];
        let words = ["oath", "pea", "eat", "rain"];
        let expected = ["eat", "oath"];
        let board = board
            .iter()
            .map(|row| row.iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();
        let words = words.iter().map(|w| w.to_string()).collect();
        let mut expected = expected.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        expected.sort();
        let mut result = Solution::find_words(board, words);
        result.sort();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let board = [["a", "b"], ["c", "d"]];
        let words = ["abcb"];
        let expected: Vec<&str> = Vec::new();
        let board = board
            .iter()
            .map(|row| row.iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();
        let words = words.iter().map(|w| w.to_string()).collect();
        let mut expected = expected.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        expected.sort();
        let mut result = Solution::find_words(board, words);
        result.sort();
        assert_eq!(expected, result);
    }

    #[test]
    fn case3() {
        let board = [["a", "b"]];
        let words = ["ba"];
        let expected = ["ba"];
        let board = board
            .iter()
            .map(|row| row.iter().map(|s| s.chars().next().unwrap()).collect())
            .collect();
        let words = words.iter().map(|w| w.to_string()).collect();
        let mut expected = expected.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        expected.sort();
        let mut result = Solution::find_words(board, words);
        result.sort();
        assert_eq!(expected, result);
    }
}
