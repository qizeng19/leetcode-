/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
var addTwoNumbers = function (l1, l2) {
  let head = new ListNode("head");
  let n = 0;
  let tmp = head;
  let sum = 0;

  while (l1 || l2) {
    let val1 = l1 ? l1.val : 0;
    let val2 = l2 ? l2.val : 0;

    sum = val1 + val2 + n;

    tmp.next = new ListNode(sum % 10); // 注意这里是对10 求余数
    tmp = tmp.next;

    n = sum >= 10 ? 1 : 0;

    if (l1) l1 = l1.next;
    if (l2) l2 = l2.next;
  }

  if (n > 0) tmp.next = new ListNode(n);
  return head.next;
};
