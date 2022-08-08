pub struct Solution;

impl Solution {
    pub fn merge_similar_items(
        mut items1: Vec<Vec<i32>>,
        mut items2: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        items1.sort_unstable();
        items2.sort_unstable();
        let mut i = 0;
        let mut j = 0;
        let mut result = Vec::new();
        while i < items1.len() && j < items2.len() {
            match items1[i][0].cmp(&items2[j][0]) {
                std::cmp::Ordering::Less => {
                    result.push(items1[i].to_vec());
                    i += 1;
                }
                std::cmp::Ordering::Equal => {
                    result.push(vec![items1[i][0], items1[i][1] + items2[j][1]]);
                    i += 1;
                    j += 1;
                }
                std::cmp::Ordering::Greater => {
                    result.push(items2[j].to_vec());
                    j += 1;
                }
            }
        }
        if i < items1.len() {
            result.extend_from_slice(&items1[i..]);
        } else {
            result.extend_from_slice(&items2[j..]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let items1 = [[1, 1], [4, 5], [3, 8]]
            .iter()
            .map(|i| i.to_vec())
            .collect();
        let items2 = [[3, 1], [1, 5]].iter().map(|i| i.to_vec()).collect();
        let expected = [[1, 6], [3, 9], [4, 5]]
            .iter()
            .map(|i| i.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::merge_similar_items(items1, items2));
    }
}
