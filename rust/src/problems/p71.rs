pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut r = Vec::new();
        path.split('/')
            .filter(|p| !p.is_empty() && *p != ".")
            .for_each(|p| {
                if p == ".." {
                    r.pop();
                } else {
                    r.push(p);
                }
            });
        "/".to_string() + &r.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let path = "/a/./b/../..//c/".to_string();
        assert_eq!("/c", &Solution::simplify_path(path));
    }
}
