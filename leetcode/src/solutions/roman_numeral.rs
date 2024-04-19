pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut int = 0;
        let mut prev = None;
        for c in s.chars().map(|c| convert2int(c)) {
            int += c;
            if prev.is_some() {
                if prev.unwrap() == 1 && (c == 5 || c == 10) {
                    int -= 2;

                }
                if prev.unwrap() == 10 && (c == 50 || c == 100) {
                    int -= 20;
                }
                if prev.unwrap() == 100 && (c == 1000 || c == 500) {
                    int -= 200;
                }
            }
            prev = Some(c);
        }
        int
    }
}

fn convert2int(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm_test() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}