use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let l1 = list1
            .iter()
            .enumerate()
            .map(|(i, e)| (e, i))
            .collect::<HashMap<_, _>>();
        let mut result = vec![];
        let mut min = usize::MAX;
        for (i, e) in list2.iter().enumerate() {
            if let Some(idx) = l1.get(e) {
                match (i + idx).cmp(&min) {
                    std::cmp::Ordering::Less => {
                        result.clear();
                        result.push(e.to_string());
                        min = i + idx;
                    }
                    std::cmp::Ordering::Equal => {
                        result.push(e.to_string());
                    }
                    std::cmp::Ordering::Greater => {}
                }
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
        let list1 = ["Shogun", "Tapioca Express", "Burger King", "KFC"];
        let list2 = [
            "Piatti",
            "The Grill at Torrey Pines",
            "Hungry Hunter Steakhouse",
            "Shogun",
        ];
        let expected = ["Shogun"];
        let list1 = list1.iter().map(|e| e.to_string()).collect();
        let list2 = list2.iter().map(|e| e.to_string()).collect();
        let expected = expected.iter().map(|e| e.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::find_restaurant(list1, list2));
    }

    #[test]
    fn case2() {
        let list1 = ["Shogun", "Tapioca Express", "Burger King", "KFC"];
        let list2 = ["KFC", "Shogun", "Burger King"];
        let expected = ["Shogun"];
        let list1 = list1.iter().map(|e| e.to_string()).collect();
        let list2 = list2.iter().map(|e| e.to_string()).collect();
        let expected = expected.iter().map(|e| e.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::find_restaurant(list1, list2));
    }
}
