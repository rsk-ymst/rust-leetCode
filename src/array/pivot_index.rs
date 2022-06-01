impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum: i32 = nums.iter().sum();
        let mut left_sum: i32 = 0;

        for (i, &num) in nums.iter().enumerate() {
            sum -= num;

            if left_sum == sum {
                return i as i32;
            }

            left_sum += num;
        }

        -1
    }
}
