pub struct Solution;

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut s = Vec::with_capacity(n as usize);
        let mut result = vec![0; n as usize];
        for log in logs {
            let mut parts = log.split(':');
            let id = parts.next().unwrap().parse::<usize>().unwrap();
            let label = parts.next().unwrap();
            let timestamp = parts.next().unwrap().parse::<i32>().unwrap();
            if label == "start" {
                if let Some(&(id, start)) = s.last() {
                    result[id] += timestamp - start;
                }
                s.push((id, timestamp));
            } else {
                let (_, start) = s.pop().unwrap();
                result[id] += timestamp - start + 1;
                if let Some((id, _)) = s.pop() {
                    s.push((id, timestamp + 1));
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
        let n = 2;
        let logs = ["0:start:0", "1:start:2", "1:end:5", "0:end:6"];
        let logs = logs.iter().map(|l| l.to_string()).collect();
        assert_eq!(vec![3, 4], Solution::exclusive_time(n, logs));
    }

    #[test]
    fn case2() {
        let n = 2;
        let logs = [
            "0:start:0",
            "0:start:2",
            "0:end:5",
            "1:start:6",
            "1:end:6",
            "0:end:7",
        ];
        let logs = logs.iter().map(|l| l.to_string()).collect();
        assert_eq!(vec![7, 1], Solution::exclusive_time(n, logs));
    }
}
