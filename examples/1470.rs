struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mid_idx = nums.len()/2;
        let (first, second) = nums.split_at(mid_idx);
        let mut ans = Vec::with_capacity(nums.len());

        for i in 0..mid_idx {
            ans.push(first[i]);
            ans.push(second[i]);
        }

        ans
    }
}

pub fn main() {}