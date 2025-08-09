use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_unstable_by_key(|c| c[1]);
        let mut q = BinaryHeap::new();
        let mut total = 0;
        for c in courses {
            let t = c[0];
            let d = c[1];
            if total + t <= d {
                total += t;
                q.push(t);
            } else if let Some(&tm) = q.peek()
                && tm > t
            {
                total -= tm - t;
                q.pop();
                q.push(t);
            }
        }
        q.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let courses = [[100, 200], [200, 1300], [1000, 1250], [2000, 3200]];
        let courses = courses.iter().map(|c| c.to_vec()).collect();
        assert_eq!(3, Solution::schedule_course(courses));
    }

    #[test]
    fn case2() {
        let courses = [[1, 2]];
        let courses = courses.iter().map(|c| c.to_vec()).collect();
        assert_eq!(1, Solution::schedule_course(courses));
    }

    #[test]
    fn case3() {
        let courses = [[3, 2], [4, 3]];
        let courses = courses.iter().map(|c| c.to_vec()).collect();
        assert_eq!(0, Solution::schedule_course(courses));
    }
}
