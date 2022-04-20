pub struct Solution;

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut path: Vec<&str> = vec![];
        let mut result = 0;
        let mut prefix_length = 0;
        for line in input.lines() {
            let entry = line.trim_start_matches('\t');
            let depth = line.len() - entry.len();
            while path.len() > depth {
                prefix_length -= path.pop().unwrap().len();
            }
            if entry.contains('.') {
                result = result.max(prefix_length + entry.len() + path.len());
            } else {
                path.push(entry);
                prefix_length += entry.len();
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            20,
            Solution::length_longest_path("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".into())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            32,
            Solution::length_longest_path("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext".into())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::length_longest_path("dir".into()));
    }

    #[test]
    fn case4() {
        assert_eq!(
            16,
            Solution::length_longest_path("dir\n        file.txt".into())
        );
    }
}
