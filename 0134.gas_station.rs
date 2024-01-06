impl Solution {
  pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let diff: Vec<i32> = gas
      .into_iter()
      .zip(cost.into_iter())
      .map(|(a, b)| a - b)
      .collect();

    let sum: i32 = diff.iter().sum();
    if sum < 0 {
      return -1;
    }

    let mut total = 0;
    let mut result = 0;

    for (idx, d) in diff.iter().enumerate() {
      total += d;

      if total < 0 {
        total = 0;
        result = idx + 1;
      }
    }

    result as i32
  }
}