use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut distinct_nums = HashSet::new();

        for num in &nums {
            if distinct_nums.contains(num) {
                return true;
            }

            distinct_nums.insert(*num);
        }

        false
    }
}
