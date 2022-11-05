pub struct Solution;

#[derive(Default)]
struct Trie {
    next: [Option<Box<Trie>>; 26],
    word: Option<String>,
}

impl Trie {
    fn from_words(words: Vec<String>) -> Self {
        let mut t = Self::default();
        for w in words {
            t.insert(w);
        }
        t
    }

    fn insert(&mut self, word: String) {
        let mut t = self;
        for b in word.as_bytes() {
            t = t.next[(b - b'a') as usize].get_or_insert_with(Default::default);
        }
        t.word = Some(word);
    }
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        fn bt(
            board: &mut Vec<Vec<char>>,
            i: i32,
            j: i32,
            trie: &mut Trie,
            result: &mut Vec<String>,
        ) {
            if i < 0 || j < 0 || i >= board.len() as i32 || j >= board[0].len() as i32 {
                return;
            }

            let c = board[i as usize][j as usize];
            if c == '{' {
                return;
            }

            let idx = (c as u8 - b'a') as usize;
            if let Some(t) = &mut trie.next[idx] {
                if let Some(word) = t.word.take() {
                    result.push(word);
                }
                board[i as usize][j as usize] = '{';
                bt(board, i - 1, j, t, result);
                bt(board, i + 1, j, t, result);
                bt(board, i, j - 1, t, result);
                bt(board, i, j + 1, t, result);
                board[i as usize][j as usize] = c;
            }
        }

        let mut trie = Trie::from_words(words);

        let m = board.len();
        let n = board[0].len();

        let mut result = Vec::new();
        for i in 0..m {
            for j in 0..n {
                bt(&mut board, i as i32, j as i32, &mut trie, &mut result);
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
