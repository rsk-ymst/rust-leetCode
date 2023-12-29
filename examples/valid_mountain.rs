

struct Solution {}

impl Solution {

    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let mut buf = -1;
        let mut switch_index: Option<i32> = None;

        for (i, &e) in arr.iter().enumerate() {
            if buf >= e {
                println!("{}", e);

                if switch_index.is_none() {
                    switch_index = Some(e);
                } else {
                    return false;
                }
            }

            buf = e;
        }

        if let Some(idx) = switch_index {
            if idx != 0 && idx != arr.len() as i32 {
                return true;
            }
        }

        false
    }
}

pub fn main() {
    let _x = Solution::valid_mountain_array(vec![1, 3, 2]);
    // valid_mountain_array(vec![1, 2, 3])
}
