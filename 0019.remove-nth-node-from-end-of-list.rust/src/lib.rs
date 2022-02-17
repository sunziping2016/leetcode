/// https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let sentinel = ListNode { val: 0, next: head };
        let mut ptr1 = &sentinel.next;
        for _ in 0..n {
            ptr1 = &ptr1.as_ref().unwrap().next;
        }
        let mut ptr2 = &sentinel;
        while ptr1.is_some() {
            ptr1 = &ptr1.as_ref().unwrap().next;
            ptr2 = ptr2.next.as_ref().unwrap();
        }
        let ptr = ptr2 as *const _ as *mut ListNode;
        let ptr = unsafe { ptr.as_mut() }.unwrap();
        let next = ptr.next.as_mut().unwrap().next.take();
        ptr.next = next;
        sentinel.next
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
        assert_eq!(Solution::remove_nth_from_end(list![1], 1), list![]);
        assert_eq!(
            Solution::remove_nth_from_end(list![1, 2, 3, 4, 5], 2),
            list![1, 2, 3, 5]
        );
        assert_eq!(Solution::remove_nth_from_end(list![1], 1), list![]);
        assert_eq!(Solution::remove_nth_from_end(list![1, 2], 1), list![1]);
    }
}
