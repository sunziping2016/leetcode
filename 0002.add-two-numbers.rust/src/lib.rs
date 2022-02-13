/// https://leetcode-cn.com/problems/add-two-numbers/
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head = ListNode::new(0);
        let mut tail = &mut head;
        let mut k = 0;
        while l1.is_some() || l2.is_some() || k != 0 {
            let num = l1.as_ref().map(|x| x.val).unwrap_or(0)
                + l2.as_ref().map(|x| x.val).unwrap_or(0)
                + k;
            tail.next = Some(Box::new(ListNode::new(num % 10)));
            tail = tail.next.as_mut().unwrap();
            l1 = l1.and_then(|x| x.next);
            l2 = l2.and_then(|x| x.next);
            k = num / 10;
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
            Solution::add_two_numbers(list![2, 4, 3], list![5, 6, 4]),
            list![7, 0, 8]
        );
        assert_eq!(Solution::add_two_numbers(list![0], list![0]), list![0]);
        assert_eq!(
            Solution::add_two_numbers(list![9, 9, 9, 9, 9, 9, 9], list![9, 9, 9, 9]),
            list![8, 9, 9, 9, 0, 0, 0, 1]
        );
    }
}
