pub struct Solution;

impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        let mut users = vec![0; number_of_users as usize];
        let mut ans = vec![0; number_of_users as usize];
        let mut events: Vec<_> = events
            .iter()
            .map(|v| {
                let kind = match v[0].as_str() {
                    "MESSAGE" => 1,
                    _ => 0,
                };
                (kind, v[1].parse::<u32>().unwrap(), &v[2])
            })
            .collect();
        events.sort_unstable_by_key(|e| (e.1, e.0));
        for (kind, t, mention) in events {
            if kind == 1 {
                match mention.as_str() {
                    "ALL" => {
                        ans.iter_mut().for_each(|e| *e += 1);
                    }
                    "HERE" => {
                        for (i, &online) in users.iter().enumerate() {
                            if online <= t {
                                ans[i] += 1;
                            }
                        }
                    }
                    _ => {
                        for id in mention
                            .split_ascii_whitespace()
                            .map(|id| id[2..].parse::<usize>().unwrap())
                        {
                            ans[id] += 1;
                        }
                    }
                }
            } else {
                let id: usize = mention.parse().unwrap();
                users[id] = t + 60;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let events = [
            ["MESSAGE", "10", "id1 id0"],
            ["OFFLINE", "11", "0"],
            ["MESSAGE", "71", "HERE"],
        ]
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();
        assert_eq!(vec![2, 2], Solution::count_mentions(2, events));
    }

    #[test]
    fn case2() {
        let events = [
            ["MESSAGE", "10", "id1 id0"],
            ["OFFLINE", "11", "0"],
            ["MESSAGE", "12", "ALL"],
        ]
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();
        assert_eq!(vec![2, 2], Solution::count_mentions(2, events));
    }

    #[test]
    fn case3() {
        let events = [["OFFLINE", "10", "0"], ["MESSAGE", "12", "HERE"]]
            .iter()
            .map(|v| v.iter().map(|s| s.to_string()).collect())
            .collect();
        assert_eq!(vec![0, 1], Solution::count_mentions(2, events));
    }
}
