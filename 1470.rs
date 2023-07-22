impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        for i in 0..nums.len() - n as usize {
            res.push(nums[i]);
            res.push(nums[i + n as usize]);
        }
        res
    }
}
