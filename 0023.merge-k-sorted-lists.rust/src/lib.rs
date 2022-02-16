/// https://leetcode-cn.com/problems/merge-k-sorted-lists/
pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn min_heaplify(lists: &mut [Option<Box<ListNode>>], current: usize) {
        let left = 2 * current + 1;
        let right = 2 * current + 2;
        let left_value = lists
            .get(left)
            .and_then(Option::as_ref)
            .map(|x| x.val)
            .unwrap_or(i32::MAX);
        let right_value = lists
            .get(right)
            .and_then(Option::as_ref)
            .map(|x| x.val)
            .unwrap_or(i32::MAX);
        let (child, child_value) = if left_value <= right_value {
            (left, left_value)
        } else {
            (right, right_value)
        };
        let current_value = lists[current].as_ref().map(|x| x.val).unwrap_or(i32::MAX);
        if current_value > child_value {
            lists.swap(current, child);
            if child_value != i32::MAX {
                Solution::min_heaplify(lists, child)
            }
        }
    }

    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        if lists.len() == 1 {
            return lists.swap_remove(0);
        }
        for i in (0 .. lists.len() / 2).rev() {
            Solution::min_heaplify(&mut lists, i);
        }
        let mut head = ListNode::new(0);
        let mut tail = &mut head;
        while lists[0].is_some() {
            let mut next = lists[0].as_mut().unwrap().next.take();
            std::mem::swap(&mut lists[0], &mut next);
            tail.next = next;
            tail = tail.next.as_mut().unwrap();
            Solution::min_heaplify(&mut lists, 0);
        }
        head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! list {
        () => { None };
        ($head:expr $(,$rest:expr)*) => {{
            Some(Box::new(ListNode {
                val: $head,
                next: list![$($rest),*],
            }))
        }};
    }

    #[test]
    fn test_example() {
        assert_eq!(
            Solution::merge_k_lists(vec![list![1, 4, 5], list![1, 3, 4], list![2, 6]]),
            list![1, 1, 2, 3, 4, 4, 5, 6]
        );
        assert_eq!(Solution::merge_k_lists(vec![]), list![]);
        assert_eq!(Solution::merge_k_lists(vec![list![]]), list![]);
    }
}
