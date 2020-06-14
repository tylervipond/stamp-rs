#![allow(dead_code)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

fn transpose<T: Copy>(original_pattern: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut pattern = vec![Vec::with_capacity(original_pattern.len()); original_pattern[0].len()];
    for r in original_pattern {
        for i in 0..r.len() {
            pattern[i].push(r[i]);
        }
    }
    pattern
}
fn reverse_rows<T>(pattern: &mut Vec<Vec<T>>) {
    pattern.iter_mut().for_each(|row| row.reverse());
}
fn reverse_cols<T>(pattern: &mut Vec<Vec<T>>) {
    pattern.reverse();
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Stamp<T: Clone + Copy + PartialEq> {
    pub height: u32,
    pub width: u32,
    pub pattern: Vec<Vec<T>>,
}
impl<T: Clone + Copy + PartialEq> Stamp<T> {
    pub fn new(pattern: Vec<Vec<T>>) -> Self {
        Self {
            height: pattern.len() as u32,
            width: pattern[0].len() as u32,
            pattern,
        }
    }
    pub fn rotate_90(&self) -> Self {
        let mut pattern = transpose(&self.pattern);
        reverse_rows(&mut pattern);
        Self {
            height: self.width,
            width: self.height,
            pattern,
        }
    }
    pub fn rotate_n90(&self) -> Self {
        let mut pattern = transpose(&self.pattern);
        reverse_cols(&mut pattern);
        Self {
            height: self.width,
            width: self.height,
            pattern,
        }
    }
    pub fn rotate_180(&self) -> Self {
        self.rotate_90().rotate_90()
    }
    pub fn stamp(&mut self, stamp: &Stamp<T>, pos_x: usize, pos_y: usize) {
        for y in pos_y..pos_y + stamp.height as usize {
            for x in pos_x..pos_x + stamp.width as usize {
                self.pattern[y][x] = stamp.pattern[y - pos_y][x - pos_x];
            }
        }
    }
    pub fn find(&self, query: &Stamp<T>) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        let query_height = query.height;
        let query_width = query.width;
        if query_height > self.height || query_height == 0 || query_width > self.width {
            return matches;
        }
        let last_y_index = self.height - (query_height - 1);
        let last_x_index = self.width - (query_width - 1);
        for y in 0..last_y_index {
            'outer: for x in 0..last_x_index {
                for (query_y, this_y) in (y..y + query_height).enumerate() {
                    for (query_x, this_x) in (x..x + query_width).enumerate() {
                        if query.pattern[query_y][query_x]
                            != self.pattern[this_y as usize][this_x as usize]
                        {
                            continue 'outer;
                        }
                    }
                }
                matches.push((x as usize, y as usize))
            }
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_rotate_90() {
        let stamp = Stamp::new(vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 0, 1],
            vec![0, 0, 0],
        ]);
        let result = stamp.rotate_90();
        let expected = Stamp::new(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 1, 1, 0]]);
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_rotate_negative_90() {
        let stamp = Stamp::new(vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 0, 1],
            vec![0, 0, 0],
        ]);
        let result = stamp.rotate_n90();
        let expected = Stamp::new(vec![vec![0, 1, 1, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_rotate_180() {
        let stamp = Stamp::new(vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 0, 1],
            vec![0, 0, 0],
        ]);
        let result = stamp.rotate_180();
        let expected = Stamp::new(vec![
            vec![0, 0, 0],
            vec![1, 0, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ]);
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_find_the_position_of_a_stamp_in_the_top_left_corner() {
        let stamp = Stamp::new(vec![
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 0, 1],
            vec![0, 0, 0],
        ]);
        let query = Stamp::new(vec![vec![1, 1], vec![1, 0]]);
        let result = stamp.find(&query);
        let expected = vec![(0, 0)];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_find_the_position_of_a_stamp_in_the_top_right_corner() {
        let stamp = Stamp::new(vec![
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 0, 1],
            vec![0, 0, 0],
        ]);
        let query = Stamp::new(vec![vec![1, 0], vec![0, 1]]);
        let result = stamp.find(&query);
        let expected = vec![(1, 0)];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_find_the_position_of_a_stamp_in_the_bottom_left_corner() {
        let stamp = Stamp::new(vec![
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 0, 1],
            vec![0, 0, 0],
        ]);
        let query = Stamp::new(vec![vec![0, 0], vec![0, 0]]);
        let result = stamp.find(&query);
        let expected = vec![(0, 2)];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_find_the_position_of_a_stamp_in_the_bottom_right_corner() {
        let stamp = Stamp::new(vec![
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 0, 1],
            vec![0, 0, 0],
        ]);
        let query = Stamp::new(vec![vec![0, 1], vec![0, 0]]);
        let result = stamp.find(&query);
        let expected = vec![(1, 2)];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_find_stamps_equal_in_size() {
        let stamp = Stamp::new(vec![
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 0, 1],
            vec![0, 0, 0],
        ]);
        let query = Stamp::new(vec![
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 0, 1],
            vec![0, 0, 0],
        ]);
        let result = stamp.find(&query);
        let expected = vec![(0, 0)];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_find_multiple_stamps() {
        let stamp = Stamp::new(vec![
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 0, 1],
            vec![0, 0, 0],
        ]);
        let query = Stamp::new(vec![vec![1], vec![1]]);
        let result = stamp.find(&query);
        let expected = vec![(0, 0), (2, 1)];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_stamp_in_the_top_left_corner() {
        let mut stamp = Stamp::new(vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]);
        let replace_stamp = Stamp::new(vec![vec![1, 1], vec![1, 1]]);
        stamp.stamp(&replace_stamp, 0, 0);
        let expected = Stamp::new(vec![
            vec![1, 1, 0],
            vec![1, 1, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]);
        assert_eq!(stamp, expected);
    }
    #[test]
    fn it_should_stamp_in_the_top_right_corner() {
        let mut stamp = Stamp::new(vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]);
        let replace_stamp = Stamp::new(vec![vec![1, 1], vec![1, 1]]);
        stamp.stamp(&replace_stamp, 1, 0);
        let expected = Stamp::new(vec![
            vec![0, 1, 1],
            vec![0, 1, 1],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]);
        assert_eq!(stamp, expected);
    }
    #[test]
    fn it_should_stamp_in_the_bottom_left_corner() {
        let mut stamp = Stamp::new(vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]);
        let replace_stamp = Stamp::new(vec![vec![1, 1], vec![1, 1]]);
        stamp.stamp(&replace_stamp, 0, 2);
        let expected = Stamp::new(vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![1, 1, 0],
            vec![1, 1, 0],
        ]);
        assert_eq!(stamp, expected);
    }
    #[test]
    fn it_should_stamp_in_the_bottom_right_corner() {
        let mut stamp = Stamp::new(vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ]);
        let replace_stamp = Stamp::new(vec![vec![1, 1], vec![1, 1]]);
        stamp.stamp(&replace_stamp, 1, 2);
        let expected = Stamp::new(vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 1, 1],
            vec![0, 1, 1],
        ]);
        assert_eq!(stamp, expected);
    }
}
