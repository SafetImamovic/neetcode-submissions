use std::cmp;

impl Solution
{
        pub fn max_area(heights: Vec<i32>) -> i32
        {
                /*
                let mut l: usize = 0;
                let mut r: usize = heights.len() - 1;

                [1,7,2,5,4,7,3,6]
                 ^             ^
                 l             r
                 0             7 -> 1, 6 -> dist: 7 - 0 = 7 -> min(1, 6) = 1 -> 6 * 1 = 6;

                if (heights[r] > heights[l])
                {
                        l += 1;
                }

                [1,7,2,5,4,7,3,6]
                   ^           ^
                   l           r
                   1           7 -> 7, 6 -> dist: 7 - 1 = 5 -> min(7, 6) = 6 -> 5 * 1 = 5;
                */

                let mut l: usize = 0;
                let n: usize = heights.len();
                let mut r: usize = n - 1;
                let mut area: usize = 0;

                while l < r
                {
                        let dist: usize = r - l;

                        let min: usize = cmp::min(heights[r], heights[l]) as usize; // This gets the shortest column.

                        let _area: usize = dist * min;

                        if area < _area
                        {
                                area = _area;
                        }

                        if heights[r] > heights[l]
                        {
                                l += 1;
                        }
                        else
                        {
                                r -= 1;
                        }
                }

                area as i32
        }
}
