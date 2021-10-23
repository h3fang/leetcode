pub struct Solution;

impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort_unstable();
        students.sort_unstable();
        seats
            .iter()
            .zip(students)
            .fold(0, |acc, (se, st)| acc + (se - st).abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let seats = vec![3, 1, 5];
        let students = vec![2, 7, 4];
        assert_eq!(4, Solution::min_moves_to_seat(seats, students));
    }

    #[test]
    fn case2() {
        let seats = vec![4, 1, 5, 9];
        let students = vec![1, 3, 2, 6];
        assert_eq!(7, Solution::min_moves_to_seat(seats, students));
    }

    #[test]
    fn case3() {
        let seats = vec![2, 2, 6, 6];
        let students = vec![1, 3, 2, 6];
        assert_eq!(4, Solution::min_moves_to_seat(seats, students));
    }
}
