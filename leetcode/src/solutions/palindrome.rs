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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm_test() {
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn neg_test() {
        assert!(!Solution::is_palindrome(-121))
    }
}