pub struct Solution;

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();
        let mut result = vec![];
        let mut root = ".".to_string();
        for f in folder {
            if !f.starts_with(&root) {
                result.push(f.to_string());
                root = f + "/";
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
        let folder = ["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"]
            .iter()
            .map(|f| f.to_string())
            .collect();
        let mut expected = ["/a", "/c/d", "/c/f"]
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::remove_subfolders(folder);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let folder = ["/a", "/a/b/c", "/a/b/d"]
            .iter()
            .map(|f| f.to_string())
            .collect();
        let mut expected = ["/a"].iter().map(|f| f.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::remove_subfolders(folder);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case3() {
        let folder = ["/a/b/c", "/a/b/ca", "/a/b/d"]
            .iter()
            .map(|f| f.to_string())
            .collect();
        let mut expected = ["/a/b/c", "/a/b/ca", "/a/b/d"]
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::remove_subfolders(folder);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
