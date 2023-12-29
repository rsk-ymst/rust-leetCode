struct Solution;

impl Solution {
    pub fn remove_element(self, nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;

        while nums.len() > i {
            if nums[i] == val {
                nums.remove(i);
            } else {
                i+=1;
            }
        }
        println!("{:?}", nums);

        nums.len() as i32
    }


    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;

        while i < nums.len() - 1 {
            if nums[i] == nums[i+1] {
                nums.remove(i);
            } else {
                i += 1;
            }
        }
        // println!("{:?}", nums);

        // nums.dedup();

        nums.len() as i32
    }
}

pub fn main() {
    let s = Solution;
    let x = Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]);
    println!("{}", x);
}
