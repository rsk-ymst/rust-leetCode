struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter().filter(|&x| { x < k }).collect::<Vec<i32>>().len() as i32
    }
}

fn main() {}