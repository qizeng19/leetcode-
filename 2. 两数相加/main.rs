// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1; // ← 影子绑定为可变
        let mut l2 = l2; // ← 影子绑定为可变
        let mut carry = 0;
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy; // 指向结果链表尾部

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next; // 前进
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next; // 前进
            }

            carry = sum / 10;
            let digit = sum % 10;

            // 追加新节点
            tail.next = Some(Box::new(ListNode::new(digit)));
            // 安全地把 tail 移到新尾结点
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}
