impl Solution {
  pub fn majority_element(nums: Vec<i32>) -> i32 {
    let (mut result, mut cnt) = (0, 0);
    for &x in nums.iter() {
      if cnt == 0 {
        result = x;
        cnt = 1;
      } else {
        cnt += if result == x { 1 } else { -1 };
      }
    }
    result
  }
}
