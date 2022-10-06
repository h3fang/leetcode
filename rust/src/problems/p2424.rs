pub struct LUPrefix {
    curr: i32,
    videos: Vec<bool>,
}

impl LUPrefix {
    pub fn new(n: i32) -> Self {
        Self {
            curr: -1,
            videos: vec![false; n as usize],
        }
    }

    pub fn upload(&mut self, video: i32) {
        self.videos[(video - 1) as usize] = true;
        for i in self.curr + 1..self.videos.len() as i32 {
            if self.videos[i as usize] {
                self.curr += 1;
            } else {
                break;
            }
        }
    }

    pub fn longest(&self) -> i32 {
        self.curr + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut lup = LUPrefix::new(4);
        lup.upload(3);
        assert_eq!(0, lup.longest());
        lup.upload(1);
        assert_eq!(1, lup.longest());
        lup.upload(2);
        assert_eq!(3, lup.longest());
    }
}
