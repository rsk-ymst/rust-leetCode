struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut buf = Vec::with_capacity(nums.len());
        let mut ans = Vec::new();

        for i in nums {
            if buf.contains(&i) {
               ans.push(i)
            }
            buf.push(i)
        }

        ans
    }
}

fn main() {}