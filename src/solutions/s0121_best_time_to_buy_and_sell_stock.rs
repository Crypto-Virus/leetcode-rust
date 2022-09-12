

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.into_iter().fold((0, i32::MAX), |(mut profit, mut cost), price| {
            cost = cost.min(price);
            profit = profit.max(price-cost);
            (profit, cost)
        }).0
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_121() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }
}