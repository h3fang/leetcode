pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let (mut l, mut r) = (0, n - 1);
        while l <= r {
            let m = (r - l) / 2 + l;
            let h = (n - m) as i32;
            match citations[m].cmp(&h) {
                std::cmp::Ordering::Less => l = m + 1,
                _ => {
                    if m < 1 {
                        break;
                    } else {
                        r = m - 1;
                    }
                }
            }
        }
        (n - l) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::h_index(vec![0, 1, 3, 5, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::h_index(vec![1, 2, 100]));
    }
}
