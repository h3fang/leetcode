pub struct Solution;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut pts = vec![];
        for t in trips {
            pts.push((t[1], t[0]));
            pts.push((t[2], -t[0]));
        }
        pts.sort_unstable();
        let mut curr = 0;
        for p in pts {
            curr += p.1;
            if curr > capacity {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let trips = vec![vec![2, 1, 5], vec![3, 3, 7]];
        let capacity = 4;
        assert!(!Solution::car_pooling(trips, capacity));
    }
}
