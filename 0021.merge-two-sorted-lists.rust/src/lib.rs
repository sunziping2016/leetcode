/// https://leetcode-cn.com/problems/merge-two-sorted-lists/
pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode {
            val: 0,
            next: None,
        };
        let mut tail = &mut head;
        while list1.is_some() || list2.is_some() {
            if list1.as_ref().map(|x| x.val).unwrap_or(i32::MAX) <= list2.as_ref().map(|x| x.val).unwrap_or(i32::MAX) {
                let next = list1.as_mut().unwrap().next.take();
                tail.next = list1;
                tail = tail.next.as_mut().unwrap();
                list1 = next;
            } else {
                let next = list2.as_mut().unwrap().next.take();
                tail.next = list2;
                tail = tail.next.as_mut().unwrap();
                list2 = next;
            }
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
            Solution::merge_two_lists(list![1, 2, 4], list![1, 3, 4]),
            list![1, 1, 2, 3, 4, 4]
        );
        assert_eq!(Solution::merge_two_lists(list![], list![]), list![]);
        assert_eq!(Solution::merge_two_lists(list![], list![0]), list![0]);
    }
}
