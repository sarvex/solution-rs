impl Solution {
  pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut mx = 0;

    for i in 0..nums.len() {
      if mx < i {
        return false;
      }
      mx = mx.max(i + (nums[i]));
    }
    true
  }
}
