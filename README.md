## 关于

| 编号 | 名字                                                                                 | 难度 | 完成情况 | 语言        | 用时(ms) | 内存(MB) | 时间复杂度      | 空间复杂度             |
|-----:|:-------------------------------------------------------------------------------------|:-----|:---------|:------------|---------:|---------:|:----------------|:-----------------------|
|    1 | [两数之和](./0001.two-sum.rust/src/lib.rs)                                           | 简单 | 独立完成 | rust        |        0 |      2.1 | o(n)            | o(n)                   |
|    2 | [两数相加](./0002.add-two-numbers.rust/src/lib.rs)                                   | 中等 | 独立完成 | rust        |        0 |      2.3 | o(m + n)        | o(1)                   |
|    3 | [无重复字符的最长子串](./0003.length-of-longest-substring.rust/src/lib.rs)           | 中等 | 复习完成 | rust        |        0 |      2.1 | o(n)            | o(1)                   |
|    4 | [**寻找两个正序数组的中位数**](./0004.median-of-two-sorted-arrays.rust/src/lib.rs)   | 困难 | 查看题解 | rust        |        0 |      2.1 | o(log(m + n))   | o(1)                   |
|    5 | [**最长回文子串**](./0005.longest-palindromic-substring.rust/src/lib.rs)             | 中等 | 查看题解 | rust        |        0 |      2.1 | o(n)            | o(n)                   |
|    6 | [z 字形变换](./0006.zigzag-conversion.rust/src/lib.rs)                               | 中等 | 独立完成 | rust        |        0 |      2.2 | o(n)            | o(n)                   |
|    7 | [整数反转](./0007.reverse-integer.rust/src/lib.rs)                                   | 中等 | 独立完成 | rust        |        0 |      2.0 | o(n)            | o(1)                   |
|    8 | [字符串转换整数 (atoi)](./0008.string-to-integer-atoi.rust/src/lib.rs)               | 中等 | 独立完成 | rust        |        0 |      2.1 | o(n)            | o(1)                   |
|    9 | [回文数](./0009.palindrome-number.rust/src/lib.rs)                                   | 简单 | 独立完成 | rust        |        0 |      1.9 | o(n)            | o(1)                   |
|   10 | [**正则表达式匹配**](./0010.regular-expression-matching.rust/src/lib.rs)             | 困难 | 查看题解 | rust        |        0 |      2.1 | o(mn)           | o(n) n是模式字符串长度 |
|   11 | [盛最多水的容器](./0011.container-with-most-water.rust/src/lib.rs)                   | 中等 | 复习完成 | rust        |        8 |      3.0 | o(n)            | o(1)                   |
|   12 | [整数转罗马数字](./0012.integer-to-roman.rust/src/lib.rs)                            | 中等 | 独立完成 | rust        |        4 |      2.0 | o(n)            | o(1)                   |
|   13 | [罗马数字转整数](./0013.roman-to-integer.rust/src/lib.rs)                            | 简单 | 独立完成 | rust        |        0 |      2.1 | o(n)            | o(1)                   |
|   14 | [最长公共前缀](./0014.longest-common-prefix.rust/src/lib.rs)                         | 简单 | 独立完成 | rust        |        0 |      2.1 | o(mn)           | o(1)                   |
|   15 | [**三数之和**](./0015.3sum.rust/src/lib.rs)                                          | 中等 | 查看题解 | rust        |       20 |      3.4 | o(n^2)          | o(log n)               |
|   16 | [**最接近的三数之和**](./0016.3sum-closest.rust/src/lib.rs)                          | 中等 | 查看题解 | rust        |        0 |      2.1 | o(n^2)          | o(log n)               |
|   17 | [电话号码的字母组合](./0017.letter-combinations-of-a-phone-number.rust/src/lib.rs)   | 中等 | 独立完成 | rust        |        0 |      2.2 | o(3^n) ~ o(4^n) | o(1)                   |
|   18 | [四数之和](./0018.4sum.rust/src/lib.rs)                                              | 中等 | 独立完成 | rust        |       16 |      4.8 | o(n^2)          | o(n^2)                 |
|   19 | [删除链表的倒数第 n 个结点](./0019.remove-nth-node-from-end-of-list.rust/src/lib.rs) | 中等 | 独立完成 | unsafe rust |        0 |      1.9 | o(1)            | o(1)                   |
|   20 | [有效的括号](./0020.valid-parentheses.rust/src/lib.rs)                               | 简单 | 独立完成 | rust        |        0 |      2.0 | o(n)            | o(n)                   |
|   21 | [合并两个有序链表](./0021.merge-two-sorted-lists.rust/src/lib.rs)                    | 简单 | 独立完成 | rust        |        0 |      2.1 | o(m+n)          | o(1)                   |
|   22 | [括号生成](./0022.generate-parentheses.rust/src/lib.rs)                              | 中等 | 独立完成 | rust        |        0 |      2.2 | o(2^n)          | o(2^n)                 |
|   23 | [合并k个升序链表](./0023.merge-k-sorted-lists.rust/src/lib.rs)                       | 困难 | 独立完成 | rust        |        4 |      3.0 | o(kn log k)     | o(1)                   |
|   24 | [两两交换链表中的节点](./0024.swap-nodes-in-pairs.rust/src/lib.rs)                   | 中等 | 独立完成 | rust        |        0 |      2.0 | o(n)            | o(1)                   |
|   25 | [k 个一组翻转链表](./0025.reverse-nodes-in-k-group.rust/src/lib.rs)                  | 困难 | 独立完成 | rust        |        0 |      2.3 | O(n)            | O(1)                   |
|   26 | [删除有序数组中的重复项](./0026.remove-duplicates-from-sorted-array.rust/src/lib.rs) | 简单 |          | rust        |          |          |                 |                        |
|   27 | [](./0027..rust/src/lib.rs)                                                          | 中等 |          | rust        |          |          |                 |                        |

注：空间复杂度是说除去输出外额外的存储空间。

