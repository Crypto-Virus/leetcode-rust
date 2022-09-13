
struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut count = 0;
        nums.retain(|&x| {
            if x == 0 {
                count += 1;
                return false;
            }
            true
        });

        nums.extend(vec![0; count]);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_283() {
        let mut vec = vec![0,1,0,3,12];
        Solution::move_zeroes(&mut vec);
        assert_eq!(vec![1,3,12,0,0], vec);

        let mut vec = vec![0];
        Solution::move_zeroes(&mut vec);
        assert_eq!(vec![0], vec);
    }
}