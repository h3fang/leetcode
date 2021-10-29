pub struct Solution;

impl Solution {
    pub fn is_self_crossing(mut distance: Vec<i32>) -> bool {
        let n = distance.len();
        if n < 4 {
            return false;
        }
        let mut i = 2;
        while i < n && distance[i] > distance[i - 2] {
            i += 1;
        }
        if i == n {
            return false;
        }

        if distance[i] >= distance[i - 2] - if i >= 4 { distance[i - 4] } else { 0 } {
            distance[i - 1] -= if i < 3 { 0 } else { distance[i - 3] };
        }
        i += 1;

        while i < n && distance[i] < distance[i - 2] {
            i += 1;
        }

        i != n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::is_self_crossing(vec![2, 1, 1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::is_self_crossing(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(true, Solution::is_self_crossing(vec![1, 1, 1, 1]));
    }
}
