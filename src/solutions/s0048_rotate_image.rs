
struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();

        for i in 0..matrix.len() {
            for j in 0..i {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_48() {
        let mut matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        Solution::rotate(&mut matrix);
        assert_eq!(
            vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]],
            matrix,
        );

        let mut matrix = vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]];
        Solution::rotate(&mut matrix);
        assert_eq!(
            vec![vec![15,13,2,5],vec![14,3,4,1],vec![12,6,8,9],vec![16,7,10,11]],
            matrix,
        );
    }
}