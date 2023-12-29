


pub struct Solution {}


impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        println!("hello");
        if !haystack.contains(&needle) {
            return -1;
        }

        let mut initial_idx = 0;
        let len = haystack.len();

        for i in 0..len {
            let target_range = i..(i+needle.len());
            if target_range.end > len {
                break;
            }

            let h_sli = &haystack[target_range];
            println!("{h_sli}");

            if h_sli.contains(&needle) {
                initial_idx = i as i32;
                break;
            }
        }

        initial_idx
    }
}

pub fn main() {
    let x = Solution::str_str("abc".to_owned(), "c".to_owned());
}
