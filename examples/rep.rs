

struct Solution {}

// 邪道な解き方。まだまだ。。
impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = vec![];

        for i in 0..arr.len() {
            let mut max = -1;

            for j in i+1..arr.len() {
                if arr[j] > max {
                    max = arr[j];
                }
            }

            ret.push(max);
        }

        ret
    }
}

pub fn main() {
    let _x = Solution::replace_elements(vec![1, 3, 2]);
    println!("{:?}", _x);
}
