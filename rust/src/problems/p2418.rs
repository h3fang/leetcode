pub struct Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut ids = (0..names.len()).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|&i| -heights[i]);
        ids.into_iter().map(|i| names[i].to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let names = ["Mary", "John", "Emma"]
            .iter()
            .map(|n| n.to_string())
            .collect();
        let heights = vec![180, 165, 170];
        let expected = ["Mary", "Emma", "John"]
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::sort_people(names, heights));
    }
}
