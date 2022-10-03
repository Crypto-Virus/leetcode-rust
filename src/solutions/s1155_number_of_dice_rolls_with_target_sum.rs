
use std::collections::HashMap;


struct Solution {}

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut mem = HashMap::new();
        (Self::recurse(n, k, target, &mut mem) % (10_u64.pow(9) + 7)) as i32
    }

    fn recurse(n: i32, k: i32, target: i32, mem: &mut HashMap<(i32, i32), u64>) -> u64 {
        if n == 0 {
            if target == 0 {
                return 1;
            } else {
                return 0;
            }
        }

        if target < 0 {
            return 0;
        }

        if let Some(&res) = mem.get(&(n, target)) {
            return res;
        }

        let mut res = 0;
        for i in 1..=k {
            res += Self::recurse(n-1, k, target-i, mem);
        }
        res %= 10_u64.pow(9) + 7;
        mem.insert((n, target), res);
        res
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1155() {
        assert_eq!(1, Solution::num_rolls_to_target(1, 6, 3));
        assert_eq!(6, Solution::num_rolls_to_target(2, 6, 7));
        assert_eq!(222616187, Solution::num_rolls_to_target(30, 30, 500));
    }
}