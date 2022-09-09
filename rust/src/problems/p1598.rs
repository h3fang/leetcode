pub struct Solution;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut d = 0;
        for l in &logs {
            match l.as_str() {
                "../" => {
                    if d > 0 {
                        d -= 1;
                    }
                }
                "./" => {}
                _ => d += 1,
            }
        }
        d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let logs = ["d1/", "d2/", "../", "d21/", "./"]
            .iter()
            .map(|l| l.to_string())
            .collect();
        assert_eq!(2, Solution::min_operations(logs));
    }

    #[test]
    fn case2() {
        let logs = ["d1/", "d2/", "./", "d3/", "../", "d31/"]
            .iter()
            .map(|l| l.to_string())
            .collect();
        assert_eq!(3, Solution::min_operations(logs));
    }
}
