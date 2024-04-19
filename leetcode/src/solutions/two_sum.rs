struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![];
        let iter = nums.iter().enumerate().map(|(c, i)| (c as i32, i));
        for i in iter.clone() {
            if ans.len() == 2 {
                break;
            }
            for j in iter.clone() {
                if j.1 == i.1 && j.0 == i.0 {
                    continue;
                }
                if j.1+i.1 == target {
                    ans.push(j.0);
                    ans.push(i.0);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm_test() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![1,0]);
        assert_eq!(Solution::two_sum(vec![3,3], 6), vec![1,0]);
    }
}