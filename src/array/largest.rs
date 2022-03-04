impl MySolution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return 0; }

        let mut largest = 0;
        let mut max_idx = 0;

        let mut i = 0;
        for e in &nums {
            if *e > largest {
                largest = *e;
                max_idx = i;
            }
            i += 1;
        }


        for e in &nums {
            if largest != *e && largest < (*e) * 2 { return -1; }
        }

        return max_idx;
    }
}


impl BestSolution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
         if nums.len() < 2 {
            return 0;
        }
        let (mut first, mut second) = (-1, -1);
        let mut idx = -1;
        for (i, num) in nums.into_iter().enumerate() {
            if num > first {
                second = first;
                first = num;
                idx = i as i32;
            } else if num > second  {
                second = num;
            }
        }
        if second * 2 <= first {
            idx
        } else {
            -1
        }
    }
}
