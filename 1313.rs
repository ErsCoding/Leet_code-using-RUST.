impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
       let mut res: Vec<i32> = Vec::new();

        for i in (0..nums.len()-1).step_by(2)  {
            let mut sublist: Vec<i32> = vec![];
            for j in 0..nums[i] {
                sublist.push(nums[i + 1]);
            }
            res.append(&mut sublist);
        }
        res 
    }
}
