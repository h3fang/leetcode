use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
        for t in &tickets {
            (*map.entry(&t[0]).or_default()).push(&t[1]);
        }
        for v in map.values_mut() {
            v.sort_unstable();
        }

        let n = tickets.len();

        fn dfs(
            map: &mut HashMap<&str, Vec<&str>>,
            apt: &str,
            result: &mut Vec<String>,
            n_apt: usize,
        ) -> bool {
            result.push(apt.to_string());

            if map.contains_key(apt) {
                for i in 0..map.get(apt).unwrap().len() {
                    let e = map.get_mut(apt).unwrap().get_mut(i).unwrap();
                    if e.is_empty() {
                        continue;
                    }
                    let old = <&str>::clone(e);
                    *e = "";
                    if dfs(map, old, result, n_apt) {
                        return true;
                    } else {
                        map.get_mut(apt).unwrap()[i] = old;
                    }
                }
            }

            if result.len() == n_apt {
                true
            } else {
                result.pop();
                false
            }
        }

        let mut result = Vec::new();
        dfs(&mut map, "JFK", &mut result, n + 1);

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
