pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut map: HashMap<&str, BinaryHeap<Reverse<&str>>> = HashMap::new();
        for t in &tickets {
            (*map.entry(&t[0]).or_default()).push(Reverse(&t[1]));
        }

        fn dfs(
            map: *mut HashMap<&str, BinaryHeap<Reverse<&str>>>,
            apt: &str,
            result: &mut Vec<String>,
        ) {
            if let Some(apts) = unsafe { (*map).get_mut(apt) } {
                while let Some(Reverse(next)) = apts.pop() {
                    dfs(map, next, result);
                }
            }
            result.push(apt.to_string());
        }

        let mut result = Vec::new();
        dfs(&mut map, "JFK", &mut result);
        result.reverse();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let tickets = [
            ["MUC", "LHR"],
            ["JFK", "MUC"],
            ["SFO", "SJC"],
            ["LHR", "SFO"],
        ];
        let tickets = tickets
            .iter()
            .map(|t| t.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let expected = ["JFK", "MUC", "LHR", "SFO", "SJC"];
        let expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::find_itinerary(tickets));
    }

    #[test]
    fn case2() {
        let tickets = [
            ["JFK", "SFO"],
            ["JFK", "ATL"],
            ["SFO", "ATL"],
            ["ATL", "JFK"],
            ["ATL", "SFO"],
        ];
        let tickets = tickets
            .iter()
            .map(|t| t.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let expected = ["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"];
        let expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::find_itinerary(tickets));
    }
}
