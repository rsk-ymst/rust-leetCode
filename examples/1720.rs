
struct Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut ans = vec![];

        for i in 0..encoded.len()-1 {
            ans.push(ans[i] ^ ans[i+1]);
        }

        ans
    }
}

fn main() {}