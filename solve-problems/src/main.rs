use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut opposites: HashMap<i32, i32> = HashMap::new();

        for (index, &num) in nums.iter().enumerate() {
            let opposite = target - num;
            let index = index as i32;
            
            match opposites.get(&opposite) {
                Some(&other_index) => { return vec![index, other_index]; }
                None => { opposites.insert(num, index); }
            }        
        }
        assert!(false);
        vec![]
    }
}

fn main() {
    let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
    assert_eq!(result, vec![0, 1]);
}
