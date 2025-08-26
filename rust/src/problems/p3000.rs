pub struct Solution;

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        dimensions
            .into_iter()
            .map(|d| {
                let diag = d[0] * d[0] + d[1] * d[1];
                let area = d[0] * d[1];
                (diag, area)
            })
            .max()
            .unwrap()
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let dimensions = [[9, 3], [8, 6]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(48, Solution::area_of_max_diagonal(dimensions));
    }

    #[test]
    fn case2() {
        let dimensions = [[3, 4], [4, 3]].iter().map(|e| e.to_vec()).collect();
        assert_eq!(12, Solution::area_of_max_diagonal(dimensions));
    }
}
