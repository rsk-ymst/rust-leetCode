// Squares of a Sorted Array
// Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.


mod array;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();

        for mut e in nums {
            let buf = e.clone();
            res.push(buf.pow(2));
        }

        res.sort();
        return res;
    }
}

/*
nums が mutableなものとして宣言されていれば、以下の様に可変参照でfor文をまわせる
for e in &mut nums {
    *e = (*e).pow(2)
}
*/
