

struct Solution {}

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::format(s) == Self::format(t)
    }

    fn format(mut s: String) -> String {
        let mut ret = String::new();
        let mut backspace_stack = 0;
        while let Some(c) = s.pop() {
            match c {
                '#' => backspace_stack += 1,
                _ => {
                    if backspace_stack > 0 {
                        backspace_stack -= 1;
                    } else {
                        ret.push(c);
                    }
                }
            };
        }
        ret
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_844() {
        assert_eq!(
            true,
            Solution::backspace_compare(String::from("ab#c"), String::from("ad#c"))
        );

        assert_eq!(
            true,
            Solution::backspace_compare(String::from("ab##"), String::from("c#d#"))
        );

        assert_eq!(
            false,
            Solution::backspace_compare(String::from("a#c"), String::from("b"))
        );
    }
}