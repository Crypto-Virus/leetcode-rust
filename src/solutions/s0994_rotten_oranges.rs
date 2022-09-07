
struct Solution {}

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {

        let mut grid = grid;
        let mut fresh_oranges = Vec::new();

        for (y, row) in grid.iter().enumerate() {
            for (x, orange) in row.iter().enumerate() {
                if *orange == 1 {
                    fresh_oranges.push((x, y));
                }
            }
        }

        let mut min_counter = 0;
        while !fresh_oranges.is_empty() {
            min_counter += 1;
            let mut oranges_to_rot = Vec::new();
            fresh_oranges.retain(|&(x, y)| {
                if (
                    (x != 0 && grid[y][x-1] == 2) ||
                    (x != grid[0].len() - 1 && grid[y][x+1] == 2) ||
                    (y != 0 && grid[y-1][x] == 2) ||
                    (y != grid.len() - 1 && grid[y+1][x] == 2)
                   )
                {
                    oranges_to_rot.push((x, y));
                    return false;
                }
                true
            });
            for &(x, y) in &oranges_to_rot {
                grid[y][x] = 2;
            }
            if oranges_to_rot.is_empty() { break; }
        }

        match fresh_oranges.is_empty() {
            true => min_counter,
            false => -1,
        }

    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_994() {
        assert_eq!(
            4,
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 1], vec![0, 1, 1]])
        );

        assert_eq!(
            -1,
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]])
        );

        assert_eq!(
            0,
            Solution::oranges_rotting(vec![vec![0, 2]])
        );
    }
}