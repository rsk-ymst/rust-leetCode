use std::error::Error;

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = 0;

        for i in 0..nums.len() {
            println!("{} {}", i, nums[i]);
            if val == nums[i] {
                for j in i..nums.len()-1 {
                    nums[j] = nums[j+1];
                }
                count += 1;
            }
        }

        count
    }
}
