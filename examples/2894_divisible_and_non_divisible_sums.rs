struct Solution;

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let mut not_divisible_sum = 0;
        let mut divisible_sum = 0;

        for i in 1..=n {
            if i % m == 0 {
                divisible_sum += i
            } else {
                not_divisible_sum += i
            }
        }

        not_divisible_sum - divisible_sum
    }
}

fn main() {}