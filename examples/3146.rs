
struct Solution;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut sum = 0;
        for (first_idx, first_char) in s.chars().enumerate() {
            for (second_idx, second_char) in t.chars().enumerate() {
                if first_char == second_char {
                    let abs_value = first_idx.abs_diff(second_idx);
                    sum += abs_value;
                    break;
                }
            }
        }

        0
    }
}

pub fn main() {

}