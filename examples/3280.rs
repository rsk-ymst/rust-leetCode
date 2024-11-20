
struct Solution;

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        date.split("-").map(|s| {
            format!("{:0b}", s.parse::<usize>().unwrap())
        }).collect().join(",")
    }
}

fn main() {

}