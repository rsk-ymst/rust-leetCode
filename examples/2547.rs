struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();

        let mut left_sum = Vec::with_capacity(nums_len);
        let mut right_sum = Vec::with_capacity(nums_len);
        let mut ans = Vec::with_capacity(nums_len);

        let mut cur_left_sum = 0;
        let mut cur_right_sum = 0;

        for i in 0..nums_len {
            left_sum.push(cur_left_sum);
            right_sum.push(cur_right_sum);
            cur_left_sum += nums[i];
            cur_right_sum += nums[(nums_len-1) - i];
        }
        right_sum.reverse();

        // println!("{:?}", left_sum);
        // println!("{:?}", right_sum);

        for i in 0..nums_len {
            ans.push(left_sum[i].abs_diff(right_sum[i]) as i32);
        }
        // println!("{:?}", ans);

        ans
    }
}

fn main() {
    Solution::left_right_difference(vec![10,4,8,3]);
}