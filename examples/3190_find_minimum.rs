struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut sum = 0;

        for i in nums {
            sum += (i - 3).abs()
        }

        sum
    }
}

fn main() {}