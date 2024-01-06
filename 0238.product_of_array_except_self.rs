impl Solution {
  pub fn product_except_self_fast(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix: Vec<_> = nums
      .iter()
      .scan(1, |state, x| {
        let val = *state;
        *state *= x;
        Some(val)
      })
      .collect();
    nums.iter()
      .enumerate()
      .rev()
      .scan(1, |state, (i, x)| {
        let val = *state;
        *state *= x;
        Some((i, val))
      })
      .for_each(|(i, postfix)| prefix[i] *= postfix);
    prefix
  }

  pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![1; n];
    for i in 1..n {
      result[i] = result[i - 1] * nums[i - 1];
    }
    let mut r = 1;
    for i in (0..n).rev() {
      result[i] *= r;
      r *= nums[i];
    }
    result
  }
}
