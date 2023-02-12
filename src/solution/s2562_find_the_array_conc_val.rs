#[solution]
impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let mut result: i64 = 0;

        let mut left = 0usize;
        let mut right = nums.len() - 1;

        while (left <= right) {
            let mut v = nums[left];

            if left < right {
                let mut num_right = nums[right];
                while (num_right > 0) {
                    v *= 10;
                    num_right /= 10;
                }

                v += nums[right];
            }

            result += v as i64;

            left += 1;
            right -= 1;
        }

        return result;
    }
}
