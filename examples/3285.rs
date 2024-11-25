struct Solution;

impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        let mut ans = vec![];
        for i in height {
            if i >= threshold {
                ans.push(i)
            }
        }
        ans
    }
}

fn main() {}