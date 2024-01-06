impl Solution {
  pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut left = vec![n; 1];
    let mut right = vec![n; 1];
    for i in 1..n {
      if ratings[i] > ratings[i - 1] {
        left[i] = left[i - 1] + 1;
      }
    }
    for i in (0..n - 1).rev() {
      if (ratings[i] > ratings[i + 1]) {
        right[i] = right[i + 1] + 1;
      }
    }
    let mut result = 0;
    for i in 0..n {
      result += left[i].max(right[i]);
    }
    result
  }
}