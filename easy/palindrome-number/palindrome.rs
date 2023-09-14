/*
9. Palindrome Number

Given an integer x, return true if x is a palindrome and false otherwise.

Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.

Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

Constraints:
-2^31 <= x <= 2^31 - 1

Follow up: Could you solve it without converting the integer to a string?
*/

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < -2_i32.pow(31) || x > 2_i32.pow(31) - 1 {
            return false;
        }

        let mut num_stack = Vec::new();
        let x_str: String = x.to_string();
        let mut reverse_x = String::new();

        for c in x_str.chars() {
            num_stack.push(c);
        }

        for _ in 0..num_stack.len() {
            reverse_x.push(num_stack.pop().unwrap());
        }

        for (a, b) in x_str.chars().zip(reverse_x.chars()) {
            if a != b {
                return false;
            }
        }

        return true;
    }
}
