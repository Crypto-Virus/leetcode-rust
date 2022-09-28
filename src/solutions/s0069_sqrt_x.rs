
struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut n = 1;
        while n <= x / n {
            n += 1;
        }
        n - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_69() {
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(2, Solution::my_sqrt(8));
    }
}