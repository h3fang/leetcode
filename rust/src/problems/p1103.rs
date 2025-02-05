pub struct Solution;

impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let mut s = (1 + num_people) * num_people / 2;
        let mut k = 0;
        while candies >= s {
            k += 1;
            candies -= s;
            s += num_people * num_people;
        }
        let mut r = (1..=num_people)
            .map(|i| (i + i + num_people * (k - 1)) * k / 2)
            .collect::<Vec<_>>();
        s = 1 + num_people * k;
        for e in r.iter_mut() {
            if candies > s {
                candies -= s;
                *e += s;
            } else {
                *e += candies;
                break;
            }
            s += 1;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![1, 2, 3, 1], Solution::distribute_candies(7, 4));
    }

    #[test]
    fn case2() {
        assert_eq!(vec![5, 2, 3], Solution::distribute_candies(10, 3));
    }
}
