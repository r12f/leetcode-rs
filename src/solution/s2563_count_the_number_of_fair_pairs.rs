#[solution]
impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        if nums.len() == 0 {
            return 0;
        }

        nums.sort();

        let range = lower..=upper;
        let mut pair_count: i64 = 0;
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;
        while left < right {
            let sum = nums[left as usize] + nums[right as usize];
            if sum > upper {
                right -= 1;
                continue;
            }

            if sum < lower {
                left += 1;
                continue;
            }

            pair_count += 1;

            let mut range_left = left + 1;
            let mut range_right = right - 1;
            let mut right_limit = None;
            while range_left <= range_right {
                let mut mid = (range_left + range_right) / 2;
                let sum = nums[left as usize] + nums[mid as usize];
                if range.contains(&sum) {
                    range_right = mid - 1;
                    right_limit = Some(mid);
                } else {
                    range_left = mid + 1;
                }
            }
            if let Some(right_limit) = right_limit {
                pair_count += (right - right_limit) as i64;
            }

            range_left = left + 1;
            range_right = right - 1;
            let mut left_limit = None;
            while range_left <= range_right {
                let mut mid = (range_left + range_right) / 2;
                let sum = nums[mid as usize] + nums[right as usize];
                if range.contains(&sum) {
                    range_left = mid + 1;
                    left_limit = Some(mid);
                } else {
                    range_right = mid - 1;
                }
            }
            if let Some(left_limit) = left_limit {
                pair_count += (left_limit - left) as i64;
            }

            left += 1;
            right -= 1;
        }

        return pair_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6355() {
        // assert_eq!(Solution::count_fair_pairs(vec![5], 3, 6), 0);
        // assert_eq!(Solution::count_fair_pairs(vec![1], 3, 6), 0);
        // assert_eq!(Solution::count_fair_pairs(vec![1, 2], 3, 6), 1);
        assert_eq!(Solution::count_fair_pairs(vec![0, 0, 0, 0, 0, 0], 0, 0), 15);
        assert_eq!(Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6), 6);
        assert_eq!(Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11), 1);
    }
}
