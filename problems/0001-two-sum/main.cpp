#include "headers.h"

class Solution {
public:
  vector<int> twoSum(vector<int> &nums, int target) {
    for (int i = 0; i < nums.size(); ++i) {
      for (int j = i + 1; j < nums.size(); ++j) {
        if (nums[i] + nums[j] == target) {
          return {i, j};
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
