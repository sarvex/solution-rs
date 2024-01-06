impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut min = i32::MAX;
    for price in prices {
      result = result.max(price - min);
      min = min.min(price);
    }
    result
  }
}
