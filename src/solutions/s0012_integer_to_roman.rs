
struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let ones = ['I', 'X', 'C', 'M'];
        let fives = ['V', 'L', 'D'];
        let mut digits = Vec::new();

        while num > 0 {
            digits.push(num % 10);
            num = num / 10;
        }

        digits.into_iter().enumerate().rev().map(|(idx, x)| {
            match x {
                1 => vec![ones[idx]],
                2 => vec![ones[idx], ones[idx]],
                3 => vec![ones[idx], ones[idx], ones[idx]],
                4 => vec![ones[idx], fives[idx]],
                5 => vec![fives[idx]],
                6 => vec![fives[idx], ones[idx]],
                7 => vec![fives[idx], ones[idx], ones[idx]],
                8 => vec![fives[idx], ones[idx], ones[idx], ones[idx]],
                9 => vec![ones[idx], ones[idx+1]],
                _ => vec![],
            }
        }).flatten().collect()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(
            String::from("III"),
            Solution::int_to_roman(3),
        );

        assert_eq!(
            String::from("LVIII"),
            Solution::int_to_roman(58),
        );

        assert_eq!(
            String::from("MCMXCIV"),
            Solution::int_to_roman(1994),
        );
    }
}