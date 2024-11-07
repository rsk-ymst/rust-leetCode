struct Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut x = address.split('.').into_iter().collect::<Vec<String>>();
        let mut ans = String::new();
        for i in 0..x.len() {
            ans.push_str(&x[i]);
            if i != x.len()-1 {
                ans.push_str("[.]");
            }
        }
        ans
    }
}

fn main() {}