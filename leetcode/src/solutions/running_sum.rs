struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
                let mut count = 0;
        let mut ans = vec![];
        for i in nums {
            count += i;
            ans.push(count);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm_test() {
        assert_eq!(Solution::running_sum(vec![1,2,3,4]), vec![1,3,6,10])
    }
}