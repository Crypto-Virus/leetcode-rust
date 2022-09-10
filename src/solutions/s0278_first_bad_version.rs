

struct Solution {
    bad_version: i32,
}

impl Solution {
    pub fn new(bad_version: i32) -> Self {
        Self { bad_version }
    }

    fn isBadVersion(&self, version: i32) -> bool {
        if version >= self.bad_version {
            true
        } else {
            false
        }
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
		let mut l = 1;
        let mut r = n;

        while l < r {
            let mid = l + (r - l) / 2;
            match self.isBadVersion(mid) {
                true => r = mid,
                false => l = mid + 1
            }
        }

        l
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_278() {
        let solution = Solution::new(4);
        assert_eq!(
            4,
            solution.first_bad_version(5)
        );

        let solution = Solution::new(1);
        assert_eq!(
            1,
            solution.first_bad_version(1)
        );
    }
}