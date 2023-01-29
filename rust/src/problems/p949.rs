pub struct Solution;

impl Solution {
    pub fn largest_time_from_digits(mut arr: Vec<i32>) -> String {
        arr.sort_unstable();

        for i in (0..4).rev() {
            for j in (0..4).rev() {
                if i == j {
                    continue;
                }
                let h = arr[i] * 10 + arr[j];
                if !(0..24).contains(&h) {
                    continue;
                }

                let mut m1 = 3;
                while m1 == i || m1 == j {
                    m1 -= 1;
                }
                let mut m2 = 3;
                while m2 == i || m2 == j || m2 == m1 {
                    m2 -= 1;
                }

                let ms = [arr[m1] * 10 + arr[m2], arr[m2] * 10 + arr[m1]];
                for m in &ms {
                    if (0..60).contains(m) {
                        return format!("{h:02}:{m:02}");
                    }
                }
            }
        }

        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "23:41".to_string(),
            Solution::largest_time_from_digits(vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "".to_string(),
            Solution::largest_time_from_digits(vec![5, 5, 5, 5])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "10:00".to_string(),
            Solution::largest_time_from_digits(vec![0, 0, 1, 0])
        );
    }
}
