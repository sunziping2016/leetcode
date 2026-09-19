#include <bits/stdc++.h>
#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
  vector<int> twoSum(vector<int> &nums, int target) {
    unordered_map<int, size_t> items;
    for (size_t i = 0; i < nums.size(); ++i) {
      auto it = items.find(target - nums[i]);
      if (it != items.end())
        return {static_cast<int>(it->second), static_cast<int>(i)};
      items.emplace(nums[i], i);
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
