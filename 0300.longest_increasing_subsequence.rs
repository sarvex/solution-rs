impl Solution {
  pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut result = vec![1; n];
    for i in 1..n {
      for j in 0..i {
        if nums[j] < nums[i] {
          result[i] = result[i].max(result[j] + 1);
        }
      }
    }
    *result.iter().max().unwrap()
  }
}
