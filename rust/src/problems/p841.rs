pub struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; rooms.len()];
        visited[0] = true;
        let mut keys: Vec<i32> = Vec::with_capacity(rooms.len());
        keys.extend(&rooms[0]);
        let mut cnt = 1;
        while let Some(key) = keys.pop() {
            if visited[key as usize] {
                continue;
            }
            keys.extend(&rooms[key as usize]);
            visited[key as usize] = true;
            cnt += 1;
        }
        cnt == rooms.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let rooms = vec![vec![1], vec![2], vec![3], vec![]];
        assert!(Solution::can_visit_all_rooms(rooms));
    }

    #[test]
    fn case2() {
        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
        assert!(!Solution::can_visit_all_rooms(rooms));
    }
}
