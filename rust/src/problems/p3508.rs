use std::collections::{HashMap, HashSet, VecDeque};

pub struct Router {
    q: VecDeque<(i32, i32, i32)>,
    set: HashSet<(i32, i32, i32)>,
    map: HashMap<i32, VecDeque<(i32, i32)>>,
}

impl Router {
    pub fn new(memory_limit: i32) -> Self {
        Self {
            q: VecDeque::with_capacity(memory_limit as usize),
            set: HashSet::with_capacity(memory_limit as usize),
            map: HashMap::with_capacity(memory_limit as usize),
        }
    }

    pub fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let k = (source, destination, timestamp);
        if self.set.contains(&k) {
            false
        } else {
            if self.q.len() == self.q.capacity() {
                let (s, d, t) = self.q.pop_front().unwrap();
                self.set.remove(&(s, d, t));
                self.map.get_mut(&d).unwrap().pop_front();
            }
            self.q.push_back(k);
            self.set.insert(k);
            self.map
                .entry(destination)
                .or_default()
                .push_back((timestamp, source));
            true
        }
    }

    pub fn forward_packet(&mut self) -> Vec<i32> {
        if let Some((s, d, t)) = self.q.pop_front() {
            self.set.remove(&(s, d, t));
            self.map.get_mut(&d).unwrap().pop_front();
            vec![s, d, t]
        } else {
            vec![]
        }
    }

    pub fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        self.map
            .get(&destination)
            .map(|q| {
                let a = q.partition_point(|e| e.0 < start_time);
                let b = q.partition_point(|e| e.0 <= end_time);
                (b - a) as i32
            })
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut r = Router::new(3);
        assert!(r.add_packet(1, 4, 90));
        assert!(r.add_packet(2, 5, 90));
        assert!(!r.add_packet(1, 4, 90));
        assert!(r.add_packet(3, 5, 95));
        assert!(r.add_packet(4, 5, 105));
        assert_eq!(vec![2, 5, 90], r.forward_packet());
        assert!(r.add_packet(5, 2, 110));
        assert_eq!(1, r.get_count(5, 100, 110));
    }
}
