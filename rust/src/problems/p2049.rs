pub struct Solution;

impl Solution {
    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        let mut edges = vec![[usize::MAX; 2]; parents.len()];
        for (i, e) in parents.iter().enumerate().skip(1) {
            if edges[*e as usize][0] == usize::MAX {
                edges[*e as usize][0] = i;
            } else {
                edges[*e as usize][1] = i;
            }
        }

        let mut size = vec![1; parents.len()];

        fn get_size(edges: &mut Vec<[usize; 2]>, size: &mut Vec<i32>, i: usize) -> i32 {
            if edges[i] == [usize::MAX, usize::MAX] {
                size[i]
            } else {
                let mut s = 1;
                if edges[i][0] != usize::MAX {
                    s += get_size(edges, size, edges[i][0]);
                    edges[i][0] = usize::MAX;
                }
                if edges[i][1] != usize::MAX {
                    s += get_size(edges, size, edges[i][1]);
                    edges[i][1] = usize::MAX;
                }
                size[i] = s;
                s
            }
        }

        let es = edges.clone();

        get_size(&mut edges, &mut size, 0);

        let mut max = 0;
        let mut result = 0;

        (0..parents.len()).for_each(|i| {
            let p = parents[i];
            let mut s = 1i64;
            if p != -1 {
                s = (size[0] - size[i]) as i64;
            }

            if es[i][0] != usize::MAX {
                s *= size[es[i][0]] as i64;
            }

            if es[i][1] != usize::MAX {
                s *= size[es[i][1]] as i64;
            }

            match s.cmp(&max) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => result += 1,
                std::cmp::Ordering::Greater => {
                    max = s;
                    result = 1;
                }
            }
        });

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::count_highest_score_nodes(vec![-1, 2, 0, 2, 0]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::count_highest_score_nodes(vec![-1, 2, 0]));
    }
}
