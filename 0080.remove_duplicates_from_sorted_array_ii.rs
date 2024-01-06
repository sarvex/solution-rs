impl Solution {
  pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..nums.len() {
      if result < 2 || nums[i] != nums[result - 2] {
        nums[result] = nums[i];
        result += 1;
      }
    }
    result as i32
  }
}
