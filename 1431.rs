impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut max = 0;
        for i in &candies {
            if max < *i {
                max = *i;
            }
        }

        let mut res: Vec<bool> = Vec::new();

        for i in &candies {
            if i + extra_candies >= max {
                res.push(true);
            } else{
                res.push(false);
            }
        }
        res
    }
}
