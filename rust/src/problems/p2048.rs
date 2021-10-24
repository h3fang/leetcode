pub struct Solution;

impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let n_digit = (n as f32).log10() as i32 + 1;

        fn bt(i: i32, n_digit: i32, curr: i32, count: &mut [i32], result: &mut Vec<i32>) {
            if i == n_digit {
                for (k, c) in count.iter().enumerate() {
                    if *c > 0 && *c != k as i32 {
                        return;
                    }
                }
                result.push(curr);
            } else {
                for d in 1..=n_digit {
                    if count[d as usize] >= d || count[d as usize] + (n_digit - i) < d {
                        continue;
                    }

                    count[d as usize] += 1;
                    bt(i + 1, n_digit, curr * 10 + d, count, result);
                    count[d as usize] -= 1;
                }
            }
        }

        for k in [n_digit, n_digit + 1] {
            let mut count = [0; 10];
            let mut results = Vec::new();
            bt(0, k, 0, &mut count, &mut results);
            for d in results {
                if d > n {
                    return d;
                }
            }
        }

        panic!("impossible")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(22, Solution::next_beautiful_number(1));
    }

    #[test]
    fn case2() {
        assert_eq!(1333, Solution::next_beautiful_number(1000));
    }

    #[test]
    fn case3() {
        assert_eq!(3133, Solution::next_beautiful_number(3000));
    }

    #[test]
    fn case4() {
        assert_eq!(1, Solution::next_beautiful_number(0));
    }

    #[test]
    fn case5() {
        assert_eq!(1224444, Solution::next_beautiful_number(1000000));
    }

    #[test]
    fn case6() {
        assert_eq!(3331, Solution::next_beautiful_number(3314));
    }
}
