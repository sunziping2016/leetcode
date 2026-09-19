#include "headers.h"

class Solution {
public:
  vector<int> twoSum(vector<int> &nums, int target) {
    for (size_t i = 0; i < nums.size(); ++i) {
      for (size_t j = i + 1; j < nums.size(); ++j) {
        if (nums[i] + nums[j] == target) {
          return {static_cast<int>(i), static_cast<int>(j)};
        }
      }
    }
    return {};
  }
};

TEST(TwoSumTest, Example1) {
  Solution solution;
  vector<int> nums = {2, 7, 11, 15};
  int target = 9;
  vector<int> expected = {0, 1};
  vector<int> result = solution.twoSum(nums, target);
  EXPECT_EQ(result, expected);
}
