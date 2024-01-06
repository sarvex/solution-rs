impl Solution {
  pub fn find_median_sorted_arrays_fast(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (mut nums1, mut nums2) = if nums1.len() > nums2.len() { (nums2, nums1) } else { (nums1, nums2) };

    let (m, n) = (nums1.len(), nums2.len());
    let (mut low, mut high) = (0, m);

    while low <= high {
      let partition_x = (low + high) / 2;
      let partition_y = (m + n + 1) / 2 - partition_x;
      let max_x = if partition_x == 0 { i32::MIN } else { nums1[partition_x - 1] };
      let min_x = if partition_x == m { i32::MAX } else { nums1[partition_x] };
      let max_y = if partition_y == 0 { i32::MIN } else { nums2[partition_y - 1] };
      let min_y = if partition_y == n { i32::MAX } else { nums2[partition_y] };

      if max_x <= min_y && max_y <= min_x {
        return if (m + n) % 2 == 0 {
          (max_x.max(max_y) + min_x.min(min_y)) as f64 / 2.0
        } else {
          max_x.max(max_y) as f64
        }
      } else if max_x > min_y {
        high = partition_x - 1;
      } else {
        low = partition_x + 1;
      }
    }

    0.0
  }

  fn search(i: usize, j: usize, k: usize, nums1: &Vec<i32>, nums2: &Vec<i32>) -> f64 {
    if i >= nums1.len() { return nums2[j + k - 1].into(); }
    if j >= nums2.len() { return nums1[i + k - 1].into(); }
    if k == 1 { return (nums1[i].min(nums2[j]).into()); }

    let p = k / 2;
    let x = if i + p - 1 < nums1.len() { nums1[i + p - 1] } else { i32::MAX };
    let y = if j + p - 1 < nums2.len() { nums2[j + p - 1] } else { i32::MAX };

    if x < y { Self::search(i + p, j, k - p, nums1, nums2) } else { Self::search(i, j + p, k - p, nums1, nums2) }
  }
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let m = nums1.len();
    let n = nums2.len();
    if (m + n) % 2 == 1 { Self::search(0, 0, (m + n + 1) / 2, &nums1, &nums2) } else { (Self::search(0, 0, (m + n + 1) / 2, &nums1, &nums2) + Self::search(0, 0, (m + n + 2) / 2, &nums1, &nums2)) / 2.0 }
  }
}