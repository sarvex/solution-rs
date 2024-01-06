impl Solution {
  pub fn jump(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let [mut jump, mut last, mut result] = [0; 3];

    for i in 0..n - 1 {
      jump = jump.max(i + (nums[i] as usize));
      if last == i {
        result += 1;
        last = jump;
      }
    }
    result
  }
}