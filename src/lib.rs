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

#[derive(Debug, Clone)]
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
    pub fn rotate_n90(&mut self) -> Self {
        let mut pattern = transpose(&self.pattern);
        reverse_cols(&mut pattern);
        Self {
            height: self.width,
            width: self.height,
            pattern,
        }
    }
    pub fn rotate_180(&mut self) -> Self {
        self.rotate_90().rotate_90()
    }
    pub fn stamp(&mut self, stamp: &Stamp<T>, pos_x: usize, pos_y: usize) {
        for y in pos_y..pos_y + stamp.height as usize {
            for x in pos_x..pos_x + stamp.width as usize {
                self.pattern[y][x] = stamp.pattern[y - pos_y][x - pos_x];
            }
        }
    }
    pub fn find(&self, stamp: &Stamp<T>) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        let pattern_height = stamp.height;
        let pattern_width = stamp.width;
        if pattern_height > self.height
            || pattern_height == 0
            || pattern_width > self.width
        {
            return matches;
        }
        let last_y_index = self.width - pattern_height;
        let last_x_index = self.height - pattern_width;
        for y in 0..last_y_index {
            'outer: for x in 0..last_x_index {
                for (pattern_y, this_y) in (y..y + pattern_height).enumerate() {
                    for (pattern_x, this_x) in (x..x + pattern_width).enumerate() {
                        if stamp.pattern[pattern_y][pattern_x]
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
