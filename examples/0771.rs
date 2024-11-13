struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let jewels_token  = jewels.into_bytes();
        let stones_token = stones.into_bytes();
        let mut cnt = 0;

        for s in stones_token {
            if jewels_token.contains(&s) {
               cnt += 1;
            }
        }

        cnt
    }
}

fn main() {}