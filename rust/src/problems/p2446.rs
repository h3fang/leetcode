pub struct Solution;

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        !(event1[1] < event2[0] || event2[1] < event1[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let event1 = ["01:15", "02:00"].iter().map(|e| e.to_string()).collect();
        let event2 = ["02:00", "03:00"].iter().map(|e| e.to_string()).collect();
        assert!(Solution::have_conflict(event1, event2));
    }

    #[test]
    fn case2() {
        let event1 = ["01:00", "02:00"].iter().map(|e| e.to_string()).collect();
        let event2 = ["01:20", "03:00"].iter().map(|e| e.to_string()).collect();
        assert!(Solution::have_conflict(event1, event2));
    }

    #[test]
    fn case3() {
        let event1 = ["10:00", "11:00"].iter().map(|e| e.to_string()).collect();
        let event2 = ["14:00", "15:00"].iter().map(|e| e.to_string()).collect();
        assert!(!Solution::have_conflict(event1, event2));
    }
}
