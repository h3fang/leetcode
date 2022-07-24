pub struct Solution;

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let mut sum = 0;
        let mut cw = 0;
        let (start, dest) = if destination >= start {
            (start, destination)
        } else {
            (destination, start)
        };
        let start = start as usize;
        for (i, d) in distance.iter().enumerate() {
            if i >= start as usize && i < dest as usize {
                cw += d;
            }
            sum += d;
        }
        cw.min(sum - cw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            1,
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            3,
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 2)
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            4,
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3)
        );
    }
}
