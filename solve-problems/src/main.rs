use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut opposites: HashMap<i32, i32> = HashMap::new();

        for (index, &num) in nums.iter().enumerate() {
            let opposite = target - num;
            let index = index as i32;

            match opposites.get(&opposite) {
                Some(&other_index) => {
                    return vec![index, other_index];
                }
                None => {
                    opposites.insert(num, index);
                }
            }
        }
        assert!(false);
        vec![]
    }

    // pascal's triangle
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        assert!(num_rows >= 1 && num_rows <= 30);
        let mut triangle = Vec::<Vec<i32>>::new();
        let num_rows = num_rows as usize;
        triangle.reserve(num_rows);
        triangle.push(vec![1]);

        for row_idx in 1..num_rows {
            let mut row = Vec::<i32>::new();
            row.reserve(row_idx + 1);
            row.push(1);

            for idx in 0..(row_idx - 1) {
                let prev = triangle.last().unwrap();
                let value = prev[idx] + prev[idx + 1];
                row.push(value);
            }
            row.push(1);
            triangle.push(row);
        }
        triangle
    }
}

fn main() {
    let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
    // assert_eq!(result, vec![0, 1]);

    let triangle = Solution::generate(5);
    println!("{triangle:?}");
}
