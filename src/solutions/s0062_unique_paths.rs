use std::collections::HashMap;
use std::cmp::Ordering;


struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut mem = HashMap::new();
        Self::recurse(1, 1, n, m, &mut mem)
    }
    
    fn recurse(x: i32, y: i32, width: i32, height: i32, mem: &mut HashMap<(i32, i32), i32>) -> i32 {
        match mem.get(&(x, y)) {
            Some(&res) => res,
            None => {
                match (x.cmp(&width), y.cmp(&height)) {
                    (Ordering::Equal, Ordering::Equal) => 1,
                    (Ordering::Greater, _) => 0,
                    (_, Ordering::Greater) => 0,
                    (_, _) => {
                        let res = Self::recurse(x + 1, y, width, height, mem) + Self::recurse(x, y + 1, width, height, mem);
                        mem.insert((x, y), res);
                        res
                    }
                }
            } 
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_62() {
        assert_eq!(28, Solution::unique_paths(3, 7));
        assert_eq!(3, Solution::unique_paths(3, 2));
    }
}