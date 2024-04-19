pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
            let mut c: i64 = 0;
            let mut x: i64 = x as i64;
            if x < 0 {
                x = x * -1;
                while x > 0 {
                    c = c * 10 + (x % 10);
                    x /= 10;
                    if c <= i32::MAX as i64 && c >= i32::MIN as i64 {
                        continue;
                    } else {
                        return 0;
                    }
                }
                return (c * -1) as i32;
            } else {
                while x > 0 {
                    c = c * 10 + (x % 10);
                    x /= 10;
                    if c <= i32::MAX as i64 && c >= i32::MIN as i64 {
                        continue;
                    } else {
                        return 0;
                    }
                }
            }
            c as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn norm_test() {
        assert_eq!(Solution::reverse(123), 321);
    }
    #[test]
    fn neg_test() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn overflow_test() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }

}