
struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut x = 0;
        for i in operations {
            if i.contains("++") {
                x += 1;
            } else if i.contains("--") {
                x -= 1;
            }
        }
        x
    }
}

fn main() {

}