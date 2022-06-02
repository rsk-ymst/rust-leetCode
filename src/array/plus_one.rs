impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {

        for v in digits.iter_mut().rev() {
            v*++;
            v* %= 10; // 10 --> 0 にする

            if v* != 0 {
                return digits;
            }
        }

        let mut test: vec![1];
        test.append(digits) as Vec<i32>
        // vec![test, digits[..]].to_vec()
    }
}
