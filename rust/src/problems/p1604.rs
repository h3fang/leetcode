pub struct Solution;

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let n = key_name.len();
        let key_time = key_time
            .into_iter()
            .map(|t| {
                let h = t[..2].parse::<u32>().unwrap();
                let m = t[3..].parse::<u32>().unwrap();
                h * 60 + m
            })
            .collect::<Vec<_>>();
        let mut kvs = key_name.into_iter().zip(key_time).collect::<Vec<_>>();
        kvs.sort_unstable();
        let mut start = usize::MAX;
        let mut result = vec![];
        let mut count = 0;
        let mut i = 0;
        while i < n {
            if start == usize::MAX || kvs[i].0 != kvs[start].0 {
                count = 1;
                start = i;
            } else if kvs[i].1 - kvs[start].1 > 60 {
                start += 1;
                while kvs[i].1 - kvs[start].1 > 60 {
                    count -= 1;
                    start += 1;
                }
            } else {
                count += 1;
                if count == 3 {
                    let name = kvs[i].0.to_string();
                    while i < n && kvs[i].0 == name {
                        i += 1;
                    }
                    result.push(name);
                    continue;
                }
            }
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let key_name = ["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"]
            .iter()
            .map(|e| e.to_string())
            .collect();
        let key_time = [
            "10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00",
        ]
        .iter()
        .map(|e| e.to_string())
        .collect();
        assert_eq!(
            vec!["daniel".to_string()],
            Solution::alert_names(key_name, key_time)
        );
    }

    #[test]
    fn case2() {
        let key_name = ["alice", "alice", "alice", "bob", "bob", "bob", "bob"]
            .iter()
            .map(|e| e.to_string())
            .collect();
        let key_time = [
            "12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00",
        ]
        .iter()
        .map(|e| e.to_string())
        .collect();
        assert_eq!(
            vec!["bob".to_string()],
            Solution::alert_names(key_name, key_time)
        );
    }

    #[test]
    fn case3() {
        let key_name = ["john", "john", "john"]
            .iter()
            .map(|e| e.to_string())
            .collect();
        let key_time = ["23:58", "23:59", "00:01"]
            .iter()
            .map(|e| e.to_string())
            .collect();
        assert_eq!(
            vec![String::new(); 0],
            Solution::alert_names(key_name, key_time)
        );
    }

    #[test]
    fn case4() {
        let key_name = [
            "leslie", "leslie", "leslie", "clare", "clare", "clare", "clare",
        ]
        .iter()
        .map(|e| e.to_string())
        .collect();
        let key_time = [
            "13:00", "13:20", "14:00", "18:00", "18:51", "19:30", "19:49",
        ]
        .iter()
        .map(|e| e.to_string())
        .collect();
        assert_eq!(
            vec!["clare".to_string(), "leslie".to_string()],
            Solution::alert_names(key_name, key_time)
        );
    }
}
