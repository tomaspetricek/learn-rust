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

    const BOARD_SIZE: usize = 9;
    const GRID_SIZE: usize = 3;

    fn solve_sudoku_impl(board: &mut Vec<Vec<char>>, mut row: usize, mut col: usize) -> bool {
        if col == Self::BOARD_SIZE {
            col = 0;
            row += 1;
        }
        if row == Self::BOARD_SIZE {
            return true;
        }
        if board[row][col] != '.' {
            return Self::solve_sudoku_impl(board, row, col + 1);
        }
        let start_row = (row / Self::GRID_SIZE) * Self::GRID_SIZE;
        let start_col = (col / Self::GRID_SIZE) * Self::GRID_SIZE;

        'search: for value in '1'..='9' {
            for r in 0..Self::BOARD_SIZE {
                if board[r][col] == value {
                    continue 'search;
                }
            }
            for c in 0..Self::BOARD_SIZE {
                if board[row][c] == value {
                    continue 'search;
                }
            }
            for r in start_row..start_row + Self::GRID_SIZE {
                for c in start_col..start_col + Self::GRID_SIZE {
                    if board[r][c] == value {
                        continue 'search;
                    }
                }
            }
            board[row][col] = value;

            if Solution::solve_sudoku_impl(board, row, col + 1) {
                return true;
            }
            board[row][col] = '.';
        }
        false
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Solution::solve_sudoku_impl(board, 0, 0);
    }
}

fn main() {
    let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
    // assert_eq!(result, vec![0, 1]);

    let triangle = Solution::generate(5);
    println!("{triangle:?}");

    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    Solution::solve_sudoku(&mut board);

    let expect_board = vec![
        vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
        vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
        vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
        vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
        vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
        vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
        vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
        vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
        vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
    ];
    assert_eq!(board, expect_board);
}
