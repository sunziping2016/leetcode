#include <bits/stdc++.h>
#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
  int longestValidParentheses(string s) {
    size_t result = 0;
    size_t count = 0, start = 0;
    for (size_t i = 0; i < s.size(); ++i) {
      if (s[i] == '(') {
        ++count;
      } else if (count == 0) {
        start = i + 1;
      } else {
        --count;
        if (count == 0 && (i + 1 - start) > result)
          result = i + 1 - start;
      }
    }
    count = 0;
    start = s.size();
    for (size_t i = s.size(); i-- > 0;) {
      if (s[i] == ')') {
        ++count;
      } else if (count == 0) {
        start = i;
      } else {
        --count;
        if (count == 0 && (start - i) > result)
          result = start - i;
      }
    }
    return static_cast<int>(result);
  }
};

TEST(LongestValidParentheses, Example1) {
  Solution solution;
  string s = "(()";
  auto result = solution.longestValidParentheses(std::move(s));
  EXPECT_EQ(result, 2);
}

TEST(LongestValidParentheses, Example2) {
  Solution solution;
  string s = ")()())";
  auto result = solution.longestValidParentheses(std::move(s));
  EXPECT_EQ(result, 4);
}

TEST(LongestValidParentheses, Example3) {
  Solution solution;
  string s = "";
  auto result = solution.longestValidParentheses(std::move(s));
  EXPECT_EQ(result, 0);
}
