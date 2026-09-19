#include <bits/stdc++.h>
#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
  bool isMatch(string s, string p) {
    std::vector<bool> d(s.size() + 1), n(s.size() + 1);
    d[0] = true;
    for (size_t j = 0; j < p.size(); ++j) {
      auto ch = p[j];
      auto star = false;
      if (p.size() - j >= 2 && p[j + 1] == '*') {
        star = true;
        ++j;
      }
      n[0] = d[0] && star;
      for (size_t i = 0; i < s.size(); ++i) {
        if (!star) {
          n[i + 1] = d[i] && (ch == '.' || ch == s[i]);
          continue;
        }
        if (d[i + 1]) {
          n[i + 1] = true;
          continue;
        }
        n[i + 1] = false;
        for (size_t k = i + 1; k-- > 0 && (ch == '.' || ch == s[k]);) {
          if (d[k]) {
            n[i + 1] = true;
            break;
          }
        }
      }
      swap(d, n);
    }
    return d.back();
  }
};

TEST(RegularExpressionMatching, Example1) {
  Solution solution;
  string s = "aa", p = "a";
  auto result = solution.isMatch(s, p);
  EXPECT_FALSE(result);
}

TEST(RegularExpressionMatching, Example2) {
  Solution solution;
  string s = "aa", p = "a*";
  auto result = solution.isMatch(s, p);
  EXPECT_TRUE(result);
}

TEST(RegularExpressionMatching, Example3) {
  Solution solution;
  string s = "aa", p = ".*";
  auto result = solution.isMatch(s, p);
  EXPECT_TRUE(result);
}
