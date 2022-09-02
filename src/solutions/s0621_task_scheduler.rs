
struct Solution {}


use std::collections::HashMap;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {

        let mut map = HashMap::new();
        let tasks_len: i32 = tasks.len() as i32;

        for letter in tasks {
            *map.entry(letter).or_insert(0) += 1;
        }


        let mut values = map.into_values().collect::<Vec<i32>>();
        let m: i32 = *values.iter().max().unwrap();
        let mct: i32 = values.into_iter().filter(|&x| x == m).count() as i32;
        let answer = (m - 1) * (n + 1) + mct;
        answer.max(tasks_len)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test__621() {
        assert_eq!(
            8,
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
        );

        assert_eq!(
            6,
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0),
        );

        assert_eq!(
            16,
            Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'], 2),
        );
    }
}