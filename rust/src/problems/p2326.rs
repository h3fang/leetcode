use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn spiral_matrix(mut m: i32, mut n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let (mut i, mut j) = (0, -1);
        let mut dir = 0;
        let mut k = 0;
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut result = vec![vec![-1; n as usize]; m as usize];
        m -= 1;
        while let Some(mut h) = head {
            i += dirs[dir as usize].0;
            j += dirs[dir as usize].1;
            k += 1;
            if dir % 2 == 0 && k == n {
                dir = (dir + 1) % 4;
                n -= 1;
                k = 0;
            } else if dir % 2 == 1 && k == m {
                dir = (dir + 1) % 4;
                m -= 1;
                k = 0;
            }
            result[i as usize][j as usize] = h.val;
            head = h.next.take();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let m = 3;
        let n = 5;
        let head = ListNode::from_vec(&[3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0]);
        let expected = [[3, 0, 2, 6, 8], [5, 0, -1, -1, 1], [5, 2, 4, 9, 7]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::spiral_matrix(m, n, head));
    }
}
