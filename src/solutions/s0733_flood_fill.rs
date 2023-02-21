use std::collections::VecDeque;

/**
 * [733] Flood Fill
 *
 * An image is represented by an m x n integer grid image where image[i][j] represents
 * the pixel value of the image. You are also given three integers sr, sc, and color.
 * You should perform a flood fill on the image starting from the pixel image[sr][sc].
 * To perform a flood fill, consider the starting pixel, plus any pixels connected 4-directionally
 * to the starting pixel of the same color as the starting pixel, plus any pixels
 * connected 4-directionally to those pixels (also with the same color), and so on.
 * Replace the color of all of the aforementioned pixels with color.
 * Return the modified image after performing the flood fill.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/01/flood1-grid.jpg" style="width: 613px; height: 253px;" />
 * Input: image = [[1,1,1],[1,1,0],[1,0,1]], sr = 1, sc = 1, color = 2
 * Output: [[2,2,2],[2,2,0],[2,0,1]]
 * Explanation: From the center of the image with position (sr, sc) = (1, 1) (i.e., the red pixel), all pixels connected by a path of the same color as the starting pixel (i.e., the blue pixels) are colored with the new color.
 * Note the bottom corner is not colored 2, because it is not 4-directionally connected to the starting pixel.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: image = [[0,0,0],[0,0,0]], sr = 0, sc = 0, color = 0
 * Output: [[0,0,0],[0,0,0]]
 * Explanation: The starting pixel is already colored 0, so no changes are made to the image.
 * 
 *  
 * Constraints:
 * 
 * 	m == image.length
 * 	n == image[i].length
 * 	1 <= m, n <= 50
 * 	0 <= image[i][j], color < 2^16
 * 	0 <= sr < m
 * 	0 <= sc < n
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/flood-fill/
// discuss: https://leetcode.com/problems/flood-fill/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    //
    //     let mut image = image;
    //     let (sr, sc) = (sr as usize, sc as usize);
    //     let old_color = image[sr][sc];
    //
    //     if old_color != color {
    //         Self::dfs(&mut image, sr, sc, old_color, color);
    //     }
    //     image
    // }
    
    // fn dfs(img: &mut Vec<Vec<i32>>, sr: usize, sc: usize, old_color: i32, new_color: i32) {
    //     if img[sr][sc] == old_color {
    //
    //     }
    // }

    pub fn flood_fill_bfs(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {

        let mut image = image;

        let nrows = image.len() as i32;
        let ncols = image[0].len() as i32;

        let init_color = image[sr as usize][sc as usize];

        let mut q = VecDeque::new();
        q.push_back((sr, sc));
        let directions = vec![(0,1), (1,0), (0,-1), (-1,0)];

        while !q.is_empty() {
            let (i,j) = q.pop_front().unwrap();
            if image[i as usize][j as usize] == color {
                continue;
            }
            image[i as usize][j as usize] = color;

            for (di, dj) in directions.iter() {
                let nexti = i + di;
                let nextj = j + dj;

                if nexti < 0 || nextj < 0 || nexti >= nrows || nextj >= ncols {
                    continue;
                }
                if image[nexti as usize][nextj as usize] != init_color {
                    continue;
                }
                q.push_back((nexti, nextj));
            }
        }
        image
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_733() {
        assert_eq!(vec![vec![2,2,2], vec![2,2,0], vec![2,0,1]],
            Solution::flood_fill_bfs(vec![vec![1,1,1],vec![1,1,0],vec![2,0,1]],1,1,2));
        assert_eq!(vec![vec![0,0,0], vec![0,0,0]],
            Solution::flood_fill_bfs(vec![vec![0,0,0],vec![0,0,0]],0,0,0));
    }
}

