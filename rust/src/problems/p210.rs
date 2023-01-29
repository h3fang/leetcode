pub struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut g = vec![vec![]; num_courses as usize];
        let mut pre = vec![0; num_courses as usize];
        for p in prerequisites {
            g[p[1] as usize].push(p[0]);
            pre[p[0] as usize] += 1;
        }
        let mut result = vec![];
        let mut done = (0..num_courses)
            .filter(|i| pre[*i as usize] == 0)
            .collect::<Vec<_>>();
        while let Some(course) = done.pop() {
            result.push(course);
            for &next in &g[course as usize] {
                pre[next as usize] -= 1;
                if pre[next as usize] == 0 {
                    done.push(next);
                }
            }
        }
        if result.len() != num_courses as usize {
            vec![]
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let prerequisites = [[1, 0]];
        let prerequisites = prerequisites.iter().map(|p| p.to_vec()).collect();
        let num_courses = 2;
        assert_eq!(vec![0, 1], Solution::find_order(num_courses, prerequisites));
    }

    #[test]
    fn case2() {
        let prerequisites = [[1, 0], [2, 0], [3, 1], [3, 2]];
        let prerequisites = prerequisites.iter().map(|p| p.to_vec()).collect();
        let num_courses = 4;
        let result = Solution::find_order(num_courses, prerequisites);
        println!("result {result:?}");
        assert!([vec![0, 2, 1, 3], vec![0, 1, 2, 3]].contains(&result));
    }
}
