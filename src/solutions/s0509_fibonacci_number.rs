
struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n < 2 {
            true => n,
            false => Self::fib(n-1) + Self::fib(n-2)
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_509() {
        assert_eq!(1, Solution::fib(2));
        assert_eq!(2, Solution::fib(3));
        assert_eq!(3, Solution::fib(4));
    }
}