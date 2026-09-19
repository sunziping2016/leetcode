#include <bits/stdc++.h>
#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
  double findMedianSortedArrays(vector<int> &nums1, vector<int> &nums2) {
    auto k = (nums1.size() + nums2.size()) / 2;
    if ((nums1.size() + nums2.size()) % 2) {
      return findKth(nums1, nums2, k);
    }
    return (findKth(nums1, nums2, k - 1) + findKth(nums1, nums2, k)) / 2.0;
  }

private:
  int findKth(vector<int> &nums1, vector<int> &nums2, size_t k) {
    size_t l1 = 0, r1 = nums1.size(), l2 = 0, r2 = nums2.size();
    while (l1 != r1 && l2 != r2) {
      size_t k1 = (r1 - l1) / 2, k2 = (r2 - l2) / 2;
      size_t m1 = l1 + k1, m2 = l2 + k2;
      if (k <= k1 + k2) {
        if (nums1[m1] < nums2[m2]) {
          r2 = m2;
        } else {
          r1 = m1;
        }
      } else {
        if (nums1[m1] < nums2[m2]) {
          l1 = m1 + 1;
          k -= k1 + 1;
        } else {
          l2 = m2 + 1;
          k -= k2 + 1;
        }
      }
    }
    return l1 == r1 ? nums2[l2 + k] : nums1[l1 + k];
  }
};

TEST(MedianOfTwoSortedArray, Example1) {
  Solution solution;
  vector<int> nums1 = {1, 3};
  vector<int> nums2 = {2};
  auto result = solution.findMedianSortedArrays(nums1, nums2);
  EXPECT_DOUBLE_EQ(result, 2.0);
}

TEST(MedianOfTwoSortedArray, Example2) {
  Solution solution;
  vector<int> nums1 = {1, 2};
  vector<int> nums2 = {3, 4};
  auto result = solution.findMedianSortedArrays(nums1, nums2);
  EXPECT_DOUBLE_EQ(result, 2.5);
}
