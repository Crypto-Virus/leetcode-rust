

struct Solution {}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {


        let mut results = vec![0; num1.len() + num2.len()];

        for (idx1, d1) in num1.chars().rev().enumerate() {
            for (idx2, d2) in num2.chars().rev().enumerate() {

                let sum = d1.to_digit(10).unwrap() * d2.to_digit(10).unwrap() + results[idx1 + idx2];

                results[idx1 + idx2] = sum % 10;
                results[idx1 + idx2 + 1] += sum / 10;

             }
        }

        while results.len() > 1 && results.last() == Some(&0) {
            results.pop();
        }

        results.into_iter().rev().map(|x| x.to_string()).collect()

    }


}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_43() {
        assert_eq!(
            String::from("6"),
            Solution::multiply(String::from("2"), String::from("3"))
        );

        assert_eq!(
            String::from("56088"),
            Solution::multiply(String::from("123"), String::from("456"))
        );
    }
}