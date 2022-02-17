/// https://leetcode-cn.com/problems/reverse-nodes-in-k-group/
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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = ListNode { val: 0, next: head };
        let mut tail = unsafe { (&mut head as *mut ListNode).as_mut() };
        for _ in 0..k {
            tail = tail.and_then(|x| x.next.as_deref_mut());
        }
        let mut prev = &mut head;
        loop {
            let mut segment = tail.as_mut().unwrap().next.take();
            tail = segment
                .as_deref_mut()
                .and_then(|x| unsafe { (x as *mut ListNode).as_mut() });
            while let Some(mut current) = prev.next.take() {
                let next = current.next.take();
                current.next = segment;
                segment = Some(current);
                prev.next = next;
            }
            prev.next = segment;
            prev = prev.next.as_deref_mut().unwrap();
            for _ in 0..(k - 1) {
                tail = tail.and_then(|x| x.next.as_deref_mut());
                prev = prev.next.as_deref_mut().unwrap();
            }
            if tail.is_none() {
                break;
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
            Solution::reverse_k_group(list![1, 2, 3, 4, 5], 2),
            list![2, 1, 4, 3, 5]
        );
        assert_eq!(
            Solution::reverse_k_group(list![1, 2, 3, 4, 5], 3),
            list![3, 2, 1, 4, 5]
        );
        assert_eq!(
            Solution::reverse_k_group(list![1, 2, 3, 4, 5], 1),
            list![1, 2, 3, 4, 5]
        );
        assert_eq!(Solution::reverse_k_group(list![1], 1), list![1]);
    }
}
