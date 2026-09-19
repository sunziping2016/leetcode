#include <bits/stdc++.h>
#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
  constexpr static int N = 9;
  constexpr static int B = 3;
  void solveSudoku(vector<vector<char>> &board) {
    for (size_t i = 0; i < N; ++i) {
      for (size_t j = 0; j < N; ++j) {
        if (board[i][j] == '.') {
          positions.emplace_back(i, j);
        } else {
          auto v = board[i][j] - '1';
          rows[i][v] = true;
          cols[j][v] = true;
          boxes[i / B][j / B][v] = true;
        }
      }
    }
    search(board, 0);
  }

private:
  vector<tuple<size_t, size_t>> positions;
  array<array<bool, N>, N> rows = {};
  array<array<bool, N>, N> cols = {};
  array<array<array<bool, N>, B>, B> boxes = {};

  bool search(vector<vector<char>> &board, size_t p) {
    if (p == positions.size())
      return true;
    auto [x, y] = positions[p];
    for (auto v = 0; v < N; ++v) {
      if (rows[x][v] || cols[y][v] || boxes[x / B][y / B][v])
        continue;
      board[x][y] = v + '1';
      rows[x][v] = true;
      cols[y][v] = true;
      boxes[x / B][y / B][v] = true;
      if (search(board, p + 1))
        return true;
      rows[x][v] = false;
      cols[y][v] = false;
      boxes[x / B][y / B][v] = false;
    }
    board[x][y] = '.';
    return false;
  }
};

TEST(SodokuSolver, Example1) {
  Solution solution;
  vector<vector<char>> board = {
      // clang-format off
      {'5', '3', '.', '.', '7', '.', '.', '.', '.'},
      {'6', '.', '.', '1', '9', '5', '.', '.', '.'},
      {'.', '9', '8', '.', '.', '.', '.', '6', '.'},
      {'8', '.', '.', '.', '6', '.', '.', '.', '3'},
      {'4', '.', '.', '8', '.', '3', '.', '.', '1'},
      {'7', '.', '.', '.', '2', '.', '.', '.', '6'},
      {'.', '6', '.', '.', '.', '.', '2', '8', '.'},
      {'.', '.', '.', '4', '1', '9', '.', '.', '5'},
      {'.', '.', '.', '.', '8', '.', '.', '7', '9'},
      // clang-format on
  };
  vector<vector<char>> answer = {
      // clang-format off
      {'5', '3', '4', '6', '7', '8', '9', '1', '2'},
      {'6', '7', '2', '1', '9', '5', '3', '4', '8'},
      {'1', '9', '8', '3', '4', '2', '5', '6', '7'},
      {'8', '5', '9', '7', '6', '1', '4', '2', '3'},
      {'4', '2', '6', '8', '5', '3', '7', '9', '1'},
      {'7', '1', '3', '9', '2', '4', '8', '5', '6'},
      {'9', '6', '1', '5', '3', '7', '2', '8', '4'},
      {'2', '8', '7', '4', '1', '9', '6', '3', '5'},
      {'3', '4', '5', '2', '8', '6', '1', '7', '9'},
      // clang-format on
  };
  solution.solveSudoku(board);
  EXPECT_EQ(board, answer);
}
