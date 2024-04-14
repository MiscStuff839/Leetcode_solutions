fn main() {
    assert!(Solution::is_palindrome(121));
    assert!(!Solution::is_palindrome(-121))
}

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut c = x;
        let mut ans = 0;
        while c > 0 {
            ans = ans * 10 + (c%10);
            c /= 10;
        }
        ans == x
    }
}