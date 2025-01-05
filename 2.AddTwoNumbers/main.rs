// 2. Add Two Numbers
// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in
// reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

// Example 1:

// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [7,0,8]
// Explanation: 342 + 465 = 807.
// Example 2:

// Input: l1 = [0], l2 = [0]
// Output: [0]
// Example 3:

// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
// Output: [8,9,9,9,0,0,0,1]
 

// Constraints:

// The number of nodes in each linked list is in the range [1, 100].
// 0 <= Node.val <= 9
// It is guaranteed that the list represents a number that does not have leading zeros.
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;
        let mut result_head = ListNode::new(0);  // Nodo dummy para almacenar el resultado
        let mut current = &mut result_head;

        // Recorremos las listas l1 y l2 mientras haya nodos
        while l1.is_some() || l2.is_some() || carry != 0 {
            let val1 = l1.as_ref().map_or(0, |node| node.val);
            let val2 = l2.as_ref().map_or(0, |node| node.val);

            // Calculamos la suma
            let sum = val1 + val2 + carry;
            carry = sum / 10;
            let digit = sum % 10;  // Dígito de la suma

            // Creamos un nuevo nodo para el dígito de la suma
            current.next = Some(Box::new(ListNode::new(digit)));
            current = current.next.as_mut().unwrap();

            // Avanzamos en las listas l1 y l2
            if l1.is_some() {
                l1 = l1.unwrap().next;
            }
            if l2.is_some() {
                l2 = l2.unwrap().next;
            }
        }

        result_head.next  // Retornamos el siguiente nodo, que es la cabeza de la lista resultante
    }
}