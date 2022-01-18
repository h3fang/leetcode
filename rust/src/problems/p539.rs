pub struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        if time_points.len() >= 60 * 24 {
            return 0;
        }
        let mut tps = time_points
            .iter()
            .map(|t| {
                let hh = t[..2].parse::<i8>().unwrap();
                let mm = t[3..].parse::<i8>().unwrap();
                hh as i32 * 60 + mm as i32
            })
            .collect::<Vec<_>>();
        tps.sort_unstable();
        fn diff(t1: i32, t2: i32) -> i32 {
            let r = t2 - t1;
            if r > 60 * 12 {
                60 * 24 - r
            } else {
                r
            }
        }
        let mut min = diff(tps[0], *tps.last().unwrap());
        for w in tps.windows(2) {
            min = min.min(diff(w[0], w[1]));
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let time_points = ["23:59", "00:00"];
        let time_points = time_points.iter().map(|t| t.to_string()).collect();
        assert_eq!(1, Solution::find_min_difference(time_points));
    }

    #[test]
    fn case2() {
        let time_points = ["00:00", "23:59", "00:00"];
        let time_points = time_points.iter().map(|t| t.to_string()).collect();
        assert_eq!(0, Solution::find_min_difference(time_points));
    }
}
