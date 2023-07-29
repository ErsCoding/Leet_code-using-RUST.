impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut res = 0;

        for op in operations {
            if op == "--X" || op == "X--" {
                res -= 1;
            } else{
                res += 1;
            }
        }

        res
    }
}
