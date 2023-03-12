use std::cmp::Reverse;

#[solution]
impl Solution {
    pub fn max_score(mut nums: Vec<i32>) -> i32 {
        nums.sort_by_key(|&x| Reverse(x));

        let mut prefix_sum: i64 = 0;
        let mut count = 0;
        for n in nums {
            prefix_sum += n as i64;
            if prefix_sum > 0 {
                count += 1;
            } else {
                break;
            }
        }

        count
    }
}

// [-687767,-860350,950296,52109,510127,545329,-291223,-966728,852403,828596,456730,-483632,-529386,356766,147293,572374,243605,931468,641668,494446]
#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_c2() {
        assert_eq!(0, Solution::max_score(vec![0]));
        assert_eq!(1, Solution::max_score(vec![1]));
        assert_eq!(0, Solution::max_score(vec![-1]));

        assert_eq!(
            20,
            Solution::max_score(vec![
                -687767, -860350, 950296, 52109, 510127, 545329, -291223, -966728, 852403, 828596,
                456730, -483632, -529386, 356766, 147293, 572374, 243605, 931468, 641668, 494446
            ])
        );
    }
}
