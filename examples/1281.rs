
struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut num = n;
        let mut rem = 0;

        let mut sum_digits = 0;
        let mut product_digits = 1;

        while num != 0 {
            rem = num % 10;
            num = num / 10;

            sum_digits += rem;
            product_digits *= rem;
        }

        product_digits - sum_digits
    }
}

pub fn main() {
    let x = Solution::subtract_product_and_sum(234);
    println!("{x}");
}