
struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;

        let mut rev: i32 = 0;
        while x != 0 {
            let pop = x % 10;
            x /= 10;
            if rev > i32::MAX / 10 || (rev == i32::MAX/10 && pop > 7) { return 0; }
            if rev < i32::MIN / 10 || (rev == i32::MIN/10 && pop < -8) { return 0; }
            rev = rev * 10 + pop;
        }
        rev
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(
            321,
            Solution::reverse(123)
        );
        assert_eq!(
            -321,
            Solution::reverse(-123)
        );
        assert_eq!(
            21,
            Solution::reverse(120)
        );
    }
}