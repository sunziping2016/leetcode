/// https://leetcode-cn.com/problems/swap-nodes-in-pairs/
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
//

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let head = ListNode { val: 0, next: head };
        let mut p = &head;
        while p.next.as_ref().and_then(|x| x.next.as_ref()).is_some() {
            let prev = unsafe { (p as *const _ as *mut ListNode).as_mut() }.unwrap();
            let mut current = prev.next.take().unwrap();
            let mut next = current.next.take().unwrap();
            let rest = next.next.take();
            current.next = rest;
            next.next = Some(current);
            prev.next = Some(next);
            p = prev.next.as_ref().unwrap().next.as_ref().unwrap();
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
        assert_eq!(Solution::swap_pairs(list![1, 2, 3, 4]), list![2, 1, 4, 3]);
        assert_eq!(Solution::swap_pairs(list![]), list![]);
        assert_eq!(Solution::swap_pairs(list![1]), list![1]);
    }
}
