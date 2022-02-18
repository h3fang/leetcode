pub struct Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let a = &edges[0];
        let b = &edges[1];
        if a[0] == b[0] || a[0] == b[1] {
            a[0]
        } else {
            a[1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = [[1, 2], [2, 3], [4, 2]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(2, Solution::find_center(edges));
    }

    #[test]
    fn case2() {
        let edges = [[1, 2], [5, 1], [1, 3], [1, 4]];
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(1, Solution::find_center(edges));
    }
}
