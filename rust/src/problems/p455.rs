pub struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();
        let (mut i, n) = (0, g.len() as i32);
        for s in s {
            if i < n && g[i as usize] <= s {
                i += 1;
            }
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            1,
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3])
        );
    }
}
