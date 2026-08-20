use std::collections::HashSet;

impl Solution
{
        pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool
        {
                const RADIX: u32 = 10;
                const STRIDE: u32 = 3;
                const LEN: u32 = 9;


                let mut result: bool = true;

                // +-----+-----+-----+
                // |(0,0)|(0,1)|(0,2)| 
                // +-----+-----+-----+
                // |(1,0)|(1,1)|(1,2)| 
                // +-----+-----+-----+
                // |(2,0)|(2,1)|(2,2)|
                // +-----+-----+-----+
                let mut pos: (u32, u32) = (0, 0);

                let mut row_map: HashSet<u32> = HashSet::new();
                let mut col_map: HashSet<u32> = HashSet::new();
                let mut square_map: HashSet<u32> = HashSet::new();

                for i in 0..LEN
                {
                        for j in 0..LEN
                        {
                                if i < 3 
                                {
                                        pos.0 = 0;
                                }
                                else if i >= 3 && i < 6
                                {
                                        pos.0 = 1;
                                }
                                else if i >= 6 && i < 9
                                {
                                        pos.0 = 2;
                                }

                                if j < 3 
                                {
                                        pos.1 = 0;
                                }
                                else if j >= 3 && j < 6
                                {
                                        pos.1 = 1;
                                }
                                else if j >= 6 && j < 9
                                {
                                        pos.1 = 2;
                                }

                                let row_value = board[i as usize][j as usize].to_digit(RADIX).unwrap_or(0);
                                let col_value = board[j as usize][i as usize].to_digit(RADIX).unwrap_or(0);

                                let x = pos.1  as usize + (pos.0 * 3) as usize;
                                let y = (j % STRIDE) as usize + ((i * 3) % 9) as usize;

                                let square_value = board[x][y].to_digit(RADIX).unwrap_or(0);
                                // // println!("{} {}, {}", square_value, x, y);

                                /*
                                      0      1      2      3 4 5 6 7 8 9 
                                    0 (0, 0) (0, 1) (0, 2)
                                    1 (1, 0) (1, 1) (1, 2)
                                    2 (2, 0) (2, 1) (2, 2)
                                */

                                

                                if row_map.contains(&row_value)
                                {
                                        // // println!("Found!");
                                        result = false;
                                        break;
                                }
                                else
                                {
                                        if row_value != 0
                                        {
                                                row_map.insert(row_value);
                                        }
                                }

                                if col_map.contains(&col_value)
                                {
                                        // // println!("Found!");
                                        result = false;
                                        break;
                                }
                                else
                                {
                                        if col_value != 0
                                        {
                                                col_map.insert(col_value);
                                        }
                                }

                                if square_map.contains(&square_value)
                                {
                                        // // println!("Found!");
                                        result = false;
                                        break;
                                }
                                else
                                {
                                        if square_value != 0
                                        {
                                                square_map.insert(square_value);
                                        }
                                }
                        }

                        // println!("Row {i}: {:#?}", &row_map);
                        // println!("Col {i}: {:#?}", &col_map);
                        // println!("Squ {i}: {:#?}", &square_map);

                        row_map.clear();
                        col_map.clear();
                        square_map.clear();

                        // println!("");
                }

                result
        }
}
