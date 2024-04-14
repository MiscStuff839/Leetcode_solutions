fn main() {
    assert!(Solution::is_palindrome("aba".to_string()));
    assert!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()))
}

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s: Vec<char> = s.chars().collect();
        rem_chars(&mut s);
        let rev: Vec<char> = s.iter().cloned().rev().collect();
        s == rev
    }
}

fn rem_chars(vec: &mut Vec<char>) {
    let mut displacement = 0;
    for (i, c) in vec.clone().iter().enumerate() {
        if !c.is_alphanumeric() {
            vec.remove(i-displacement);
            displacement += 1;
        } 
        if c.is_uppercase() {
            *(vec.get_mut(i-displacement).unwrap()) = c.to_lowercase().next().unwrap();
        }
    }
}