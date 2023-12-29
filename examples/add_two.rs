
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut v = Vec::new();
        let mut sum = 0;

        for (i, &e) in nums.iter().enumerate() {
            for (i, &e) in nums.iter().enumerate() {
                sum += e;
                if sum == target {
                    v.push(i);
                    break;
                }
            }
        }

        v
    }
}

pub fn main() {

}
