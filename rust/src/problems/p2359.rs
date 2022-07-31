pub struct Solution;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();
        let mut s1 = vec![-1; n];
        let mut s2 = vec![-1; n];

        let mut n = node1;
        let mut dist = 0;
        while s1[n as usize] < 0 {
            s1[n as usize] = dist;
            n = edges[n as usize];
            if n == -1 {
                break;
            }
            dist += 1;
        }

        let mut n = node2;
        let mut dist = 0;
        while s2[n as usize] < 0 {
            s2[n as usize] = dist;
            n = edges[n as usize];
            if n == -1 {
                break;
            }
            dist += 1;
        }

        let mut max = i32::MAX;
        let mut result = -1;
        for (i, &d) in s1.iter().enumerate() {
            if d < 0 || s2[i] < 0 {
                continue;
            }
            if d.max(s2[i]) < max {
                max = d.max(s2[i]);
                result = i as i32;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let edges = vec![2, 2, 3, -1];
        let node1 = 0;
        let node2 = 1;
        assert_eq!(2, Solution::closest_meeting_node(edges, node1, node2));
    }

    #[test]
    fn case2() {
        let edges = vec![1, 2, -1];
        let node1 = 0;
        let node2 = 2;
        assert_eq!(2, Solution::closest_meeting_node(edges, node1, node2));
    }
}
