struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut sum = 0;
        let mut s = s.as_bytes();

        for i in 0..s.len()-1 {
            let diff: i32 = s[i] as i32 - s[i+1] as i32;
            let abs = Self::int_to_abs(diff);
            sum += abs;
        }

        sum
    }

    fn int_to_abs(val: i32) -> i32 {
        if val > 0 {
            val
        } else {
            -val
        }
    }
}

fn main() {}