use std::collections::HashMap;

#[solution]
impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let mut result: i64 = 0;

        let mut freq = HashMap::<i32, i32>::new();
        freq.insert(0, 1);

        let mut prefix_xor = 0;
        for n in nums {
            prefix_xor = prefix_xor ^ n;

            let count = match freq.get(&prefix_xor) {
                Some(v) => *v,
                None => 0,
            };

            freq.insert(prefix_xor, count + 1);
            result += count as i64;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_c3() {
        assert_eq!(2, Solution::beautiful_subarrays(vec![4, 3, 1, 2, 4]));
    }
}
