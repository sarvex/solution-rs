impl Solution {
  pub fn max_profit_fast(prices: Vec<i32>) -> i32 {
    if prices.is_empty() { return 0; }

    let mut profit = 0;
    for i in 1..prices.len() {
      if prices[i - 1] < prices[i] {
        profit += prices[i] - prices[i - 1];
      }
    }

    profit
  }

  pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 1..prices.len() {
      result += (0).max(prices[i] - prices[i - 1]);
    }
    result
  }
}
