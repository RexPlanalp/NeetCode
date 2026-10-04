impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        let mut result = vec![0; 2*n];
        for i in 0..n {
            result[i] = nums[i];
            result[i + n] = nums[i];
        }

        result
    }
}
