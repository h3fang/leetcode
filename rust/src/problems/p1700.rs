pub struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut s1 = students.iter().sum::<i32>();
        let mut s0 = students.len() as i32 - s1;
        for s in sandwiches {
            if s == 0 && s0 > 0 {
                s0 -= 1;
            } else if s == 1 && s1 > 0 {
                s1 -= 1;
            } else {
                break;
            }
        }
        s0 + s1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            0,
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            3,
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1])
        );
    }
}
