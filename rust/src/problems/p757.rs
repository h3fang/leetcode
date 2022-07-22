pub struct Solution;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|i| (i[0], -i[1]));
        let mut intersections = vec![vec![]; intervals.len()];
        let mut result = 0;
        for (i, iv) in intervals.iter().enumerate().rev() {
            let mut e = iv[0];
            for _ in intersections[i].len()..2 {
                result += 1;
                for j in (0..i).rev() {
                    if intervals[j][1] < e {
                        break;
                    }
                    intersections[j].push(e);
                }
                e += 1;
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
        let intervals = [[1, 3], [1, 4], [2, 5], [3, 5]];
        let intervals = intervals.iter().map(|i| i.to_vec()).collect();
        assert_eq!(3, Solution::intersection_size_two(intervals));
    }

    #[test]
    fn case2() {
        let intervals = [[1, 2], [2, 3], [2, 4], [4, 5]];
        let intervals = intervals.iter().map(|i| i.to_vec()).collect();
        assert_eq!(5, Solution::intersection_size_two(intervals));
    }
}
