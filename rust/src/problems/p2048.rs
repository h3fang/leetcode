pub struct Solution;

fn valid(count: &[i32]) -> bool {
    count
        .iter()
        .enumerate()
        .all(|(i, &c)| c == 0 || c == i as i32 + 1)
}

fn bt(n: i32, i: usize, curr: i32, count: &mut [i32], ans: &mut i32) {
    let k = count.len();
    if curr > n && valid(count) {
        *ans = (*ans).min(curr);
    }
    if i == k || curr >= *ans {
        return;
    }
    for d in 1..=k {
        if count[d - 1] >= d as i32 || count[d - 1] as usize + (k - i) < d {
            continue;
        }

        count[d - 1] += 1;
        bt(n, i + 1, curr * 10 + d as i32, count, ans);
        count[d - 1] -= 1;
    }
}

impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        let k = n.ilog10() as usize + 2;

        let mut count = vec![0; k];
        let mut ans = i32::MAX;
        bt(n, 0, 0, &mut count, &mut ans);
        ans
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
