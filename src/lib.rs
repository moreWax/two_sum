use std::collections::HashMap;

struct Solution();

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Hashmap | <num, idx>
        let mut storage: HashMap<i32, i32> = HashMap::new();

        // iterate over nums
        for (idx, num) in nums.iter().enumerate() {
            // if the num is in the HashMap return
            match storage.get(&num) {
                Some(n) => {
                    return vec![*n, idx as i32];
                }
                None => {
                    storage.insert(target-num, idx as i32);
                }
            }
        }

        // !vec[0, 0]
        vec![-1, 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(nums, target), [0, 1]);
    }

    #[test]
    fn example2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(Solution::two_sum(nums, target), [1, 2]);
    }

    #[test]
    fn example3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(Solution::two_sum(nums, target), [0, 1]);
    }

    #[test]
    fn example4() {
        let nums = vec![1, 4, 2, 5];
        let target = 7;
        assert_eq!(Solution::two_sum(nums, target), [2, 3]);
    }
}
