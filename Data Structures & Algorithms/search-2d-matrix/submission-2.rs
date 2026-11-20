impl Solution
{
        pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool
        {
                if matrix.is_empty()
                {
                        return false;
                }

                let mut l_row: usize = 0;
                let mut r_row: usize = matrix.len() - 1;
                let mut l_col: usize = 0;
                let mut r_col: usize = matrix.first().unwrap().len() - 1;

                while l_row <= r_row
                {
                        let mid_row: usize = l_row + (r_row - l_row) / 2;

                        if matrix[mid_row][r_col] == target
                        {
                                return true;
                        }

                        if matrix[mid_row][r_col] > target
                        {
                                if mid_row == 0
                                {
                                        break;
                                }

                                r_row = mid_row - 1;
                        }
                        else
                        {
                                l_row = mid_row + 1;
                        }
                }

                // l_row is now the index of the sub array
                // that contains the target.
                if l_row >= matrix.len()
                {
                        return false;
                }
                
                while l_col <= r_col
                {
                        let mid_col: usize = l_col + (r_col - l_col) / 2;

                        if matrix[l_row][mid_col] == target
                        {
                                return true;
                        }

                        if matrix[l_row][mid_col] > target
                        {
                                if mid_col == 0
                                {
                                        break;
                                }

                                r_col = mid_col - 1;
                        }
                        else
                        {
                                l_col = mid_col + 1;
                        }
                }

                false
        }
}
