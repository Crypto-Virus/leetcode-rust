
struct Solution {}

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {

        let colors: Vec<char> = colors.chars().collect();
        let mut times = Vec::new();

        let mut last_idx = 0;
        for i in 1..colors.len() {
            if colors[i] == colors[last_idx] {
                if needed_time[i] < needed_time[last_idx] {
                    times.push(needed_time[i]);
                } else {
                    times.push(needed_time[last_idx]);
                    last_idx = i;
                }
            } else {
                last_idx = i;
            }
        }

        times.into_iter().reduce(|accum, x| accum + x).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1578() {
        assert_eq!(3, Solution::min_cost(String::from("abaac"), vec![1,2,3,4,5]));
        assert_eq!(0, Solution::min_cost(String::from("abc"), vec![1,2,3]));
        assert_eq!(2, Solution::min_cost(String::from("aabaa"), vec![1,2,3,4,1]));
    }
}