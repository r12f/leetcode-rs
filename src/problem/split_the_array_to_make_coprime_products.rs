use std::collections::{HashMap, HashSet};

#[solution]
impl Solution {
    pub fn find_valid_split(nums: Vec<i32>) -> i32 {
        let mut primes = vec![2];
        'outer: for i in 3..1000000 {
            for j in 2..((i as f64).sqrt() as i32) {
                if i % j == 0 {
                    continue 'outer;
                }
            }

            primes.push(i);
        }

        let mut right_primes = HashSet::<i32>::new();
        let mut right_primes_order_stack = Vec::<(i32, usize)>::new();

        // Get all prime number showing up order from right.
        for i in (1..nums.len()).rev() {
            let mut n = nums[i];

            let mut prime_index = 0;
            while n > 1 {
                if n % primes[prime_index] == 0 {
                    let is_new = right_primes.insert(primes[prime_index]);
                    if is_new {
                        right_primes_order_stack.push((primes[prime_index], i));
                    }

                    while n % primes[prime_index] == 0 {
                        n /= primes[prime_index];
                    }
                }

                prime_index += 1;
            }
        }

        // Starting from left, check all prime number show up seq.
        let mut right = 1;
        for left in (0..nums.len() - 1) {
            let mut n = nums[left];

            let mut prime_index = 0;
            while n > 1 {
                if n % primes[prime_index] == 0 {
                    // If right side has this prime number, move right pointer back to where it doesn't.
                    if right_primes.contains(&primes[prime_index]) {
                        while let Some((right_prime_number, right_prime_index)) =
                            right_primes_order_stack.pop()
                        {
                            right_primes.remove(&right_prime_number);

                            if right_prime_number != primes[prime_index] {
                                continue;
                            }

                            // Right side is already moved out of boundary.
                            right = right_prime_index + 1;
                            if right >= nums.len() {
                                return -1;
                            }

                            break;
                        }
                    }

                    while n % primes[prime_index] == 0 {
                        n /= primes[prime_index];
                    }
                }

                prime_index += 1;
            }

            // Found the first match!
            if left == right - 1 {
                return left as i32;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_6309() {
        assert_eq!(-1, Solution::find_valid_split(vec![557663,280817,472963,156253,273349,884803,756869,763183,557663,964357,821411,887849,891133,453379,464279,574373,852749,15031,156253,360169,526159,410203,6101,954851,860599,802573,971693,279173,134243,187367,896953,665011,277747,439441,225683,555143,496303,290317,652033,713311,230107,770047,308323,319607,772907,627217,119311,922463,119311,641131,922463,404773,728417,948281,612373,857707,990589,12739,9127,857963,53113,956003,363397,768613,47981,466027,981569,41597,87149,55021,600883,111953,119083,471871,125641,922463,562577,269069,806999,981073,857707,831587,149351,996461,432457,10903,921091,119083,72671,843289,567323,317003,802129,612373]));
        assert_eq!(2, Solution::find_valid_split(vec![4, 7, 8, 15, 3, 5]));
    }
}
