#include <unordered_map>
#include <vector>

class Solution {
 public:
  std::vector<int> twoSum(std::vector<int>& nums, int target) {
    std::unordered_map<int, int> m;
    for (int i = 0;; ++i) {
      int x = nums[i];
      int y = target - x;
      if (m.count(y)) {
        return {m[y], i};
      }
      m[x] = i;
    }
  }
};
