#![allow(dead_code)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type Pattern<T> = Vec<Vec<T>>;

fn transpose<T>(v: &mut Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.drain(0..).map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn reverse_rows<T>(pattern: &mut Pattern<T>) {
    pattern.iter_mut().for_each(|row| row.reverse());
}
fn reverse_cols<T>(pattern: &mut Pattern<T>) {
    pattern.reverse();
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StampPart<T: Clone + PartialEq> {
    Use(T),
    Transparent,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum QueryStampPart<T: Clone + PartialEq> {
    Is(Box<[T]>),
    Not(Box<[T]>),
    Any,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Stamp<T: Clone + PartialEq> {
    pub pattern: Pattern<T>,
}

impl<'a, T: Clone + PartialEq> Stamp<T> {
    pub fn new(pattern: Pattern<T>) -> Self {
        Self { pattern }
    }
    pub fn rotate_90(&mut self) {
        let mut pattern = transpose(&mut self.pattern);
        reverse_rows(&mut pattern);
        self.pattern = pattern;
    }
    pub fn rotate_n90(&mut self) {
        let mut pattern = transpose(&mut self.pattern);
        reverse_cols(&mut pattern);
        self.pattern = pattern;
    }
    pub fn rotate_180(&mut self) {
        self.rotate_90();
        self.rotate_90();
    }
    pub fn flip_horizontal(&mut self) {
        reverse_rows(&mut self.pattern);
    }
    pub fn flip_vertical(&mut self) {
        reverse_cols(&mut self.pattern);
    }
    pub fn height(&self) -> usize {
        self.pattern.len()
    }
    pub fn width(&self) -> usize {
        match self.pattern.get(0) {
            Some(row) => row.len(),
            None => 0,
        }
    }
    pub fn set_at(&mut self, coord: (usize, usize), element: T) {
        self.pattern[coord.1][coord.0] = element;
    }
    pub fn get_at(&self, coord: (usize, usize)) -> Option<&T> {
        if let Some(row) = self.pattern.get(coord.1) {
            return row.get(coord.0);
        }
        None
    }
}

impl<'a, T: Clone + PartialEq> Stamp<StampPart<T>> {
    pub fn stamp(&mut self, stamp: &Stamp<StampPart<T>>, pos_x: usize, pos_y: usize) {
        let stamp_height = stamp.height();
        let stamp_width = stamp.width();
        for y in pos_y..pos_y + stamp_height {
            for x in pos_x..pos_x + stamp_width {
                let stamp_pattern_element = &stamp.pattern[y - pos_y][x - pos_x];
                if let StampPart::Use(_) = stamp_pattern_element {
                    self.pattern[y][x] = stamp_pattern_element.clone();
                }
            }
        }
    }

    fn find_at_position(
        &self,
        query: &Stamp<QueryStampPart<T>>,
        pos_x: usize,
        pos_y: usize,
    ) -> bool {
        for (query_y, this_y) in (pos_y..pos_y + query.height()).enumerate() {
            if let Some(row) = self.pattern.get(this_y) {
                for (query_x, this_x) in (pos_x..pos_x + query.width()).enumerate() {
                    if let Some(col) = &row.get(this_x) {
                        match &query.pattern[query_y][query_x] {
                            QueryStampPart::Any => {}
                            QueryStampPart::Not(q) => match col {
                                StampPart::Use(tq) if q.contains(tq) => return false,
                                _ => {}
                            },
                            QueryStampPart::Is(q) => match col {
                                StampPart::Use(tq) if q.contains(tq) => {}
                                _ => return false,
                            },
                        }
                    } else {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn find(&self, query: &Stamp<QueryStampPart<T>>) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        if query.height() > self.height() || query.width() > self.width() {
            return matches;
        }
        let last_y_index = self.height() - (query.height() - 1);
        let last_x_index = self.width() - (query.width() - 1);
        for y in 0..last_y_index {
            for x in 0..last_x_index {
                if self.find_at_position(query, x, y) {
                    matches.push((x, y))
                }
            }
        }
        matches
    }

    pub fn find_at_xy(&self, query: &Stamp<QueryStampPart<T>>, pos_x: usize, pos_y: usize) -> bool {
        self.find_at_position(query, pos_x, pos_y)
    }

    pub fn find_at_y(&self, query: &Stamp<QueryStampPart<T>>, pos_y: usize) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        if query.height() >= self.height() || query.width() >= self.width() {
            return matches;
        }
        let last_x_index = self.width() - (query.width() - 1);
        for pos_x in 0..last_x_index {
            if self.find_at_position(query, pos_x, pos_y) {
                matches.push((pos_x, pos_y))
            }
        }
        matches
    }

    pub fn find_at_x(&self, query: &Stamp<QueryStampPart<T>>, pos_x: usize) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        if query.height() >= self.height() || query.width() >= self.width() {
            return matches;
        }
        let last_y_index = self.height() - (query.height() - 1);
        for pos_y in 0..last_y_index {
            if self.find_at_position(query, pos_x, pos_y) {
                matches.push((pos_x, pos_y))
            }
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use QueryStampPart::{Any, Is, Not};
    use StampPart::{Transparent, Use};
    #[test]
    fn it_should_rotate_90() {
        let mut stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        stamp.rotate_90();
        let expected = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0), Use(0)],
            vec![Use(0), Use(1), Use(1), Use(0)],
        ]);
        assert_eq!(stamp, expected);
    }
    #[test]
    fn it_should_rotate_negative_90() {
        let mut stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        stamp.rotate_n90();
        let expected = Stamp::new(vec![
            vec![Use(0), Use(1), Use(1), Use(0)],
            vec![Use(0), Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0), Use(0)],
        ]);
        assert_eq!(stamp, expected);
    }
    #[test]
    fn it_should_rotate_180() {
        let mut stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        stamp.rotate_180();
        let expected = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(1), Use(0), Use(0)],
            vec![Use(1), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        assert_eq!(stamp, expected);
    }
    #[test]
    fn it_should_find_the_position_of_a_stamp_in_the_top_left_corner() {
        let stamp = Stamp::new(vec![
            vec![Use(1), Use(1), Use(0)],
            vec![Use(1), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let query_stamp = Stamp::new(vec![
            vec![Is(Box::new([1])), Is(Box::new([1]))],
            vec![Is(Box::new([1])), Is(Box::new([0]))],
        ]);
        let result = stamp.find(&query_stamp);
        let expected = vec![(0, 0)];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_find_the_position_of_a_stamp_in_the_top_right_corner() {
        let stamp = Stamp::new(vec![
            vec![Use(1), Use(1), Use(0)],
            vec![Use(1), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let query_stamp = Stamp::new(vec![
            vec![Is(Box::new([1])), Is(Box::new([0]))],
            vec![Is(Box::new([0])), Is(Box::new([1]))],
        ]);
        let result = stamp.find(&query_stamp);
        let expected = vec![(1, 0)];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_find_the_position_of_a_stamp_in_the_bottom_left_corner() {
        let stamp = Stamp::new(vec![
            vec![Use(1), Use(1), Use(0)],
            vec![Use(1), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let query_stamp = Stamp::new(vec![
            vec![Is(Box::new([0])), Is(Box::new([0]))],
            vec![Is(Box::new([0])), Is(Box::new([0]))],
        ]);
        let result = stamp.find(&query_stamp);
        let expected = vec![(0, 2)];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_find_the_position_of_a_stamp_in_the_bottom_right_corner() {
        let stamp = Stamp::new(vec![
            vec![Use(1), Use(1), Use(0)],
            vec![Use(1), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let query_stamp = Stamp::new(vec![
            vec![Is(Box::new([0])), Is(Box::new([1]))],
            vec![Is(Box::new([0])), Is(Box::new([0]))],
        ]);
        let result = stamp.find(&query_stamp);
        let expected = vec![(1, 2)];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_find_stamps_equal_in_size() {
        let stamp = Stamp::new(vec![
            vec![Use(1), Use(1), Use(0)],
            vec![Use(1), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let query_stamp = Stamp::new(vec![
            vec![Is(Box::new([1])), Is(Box::new([1])), Is(Box::new([0]))],
            vec![Is(Box::new([1])), Is(Box::new([0])), Is(Box::new([1]))],
            vec![Is(Box::new([0])), Is(Box::new([0])), Is(Box::new([1]))],
            vec![Is(Box::new([0])), Is(Box::new([0])), Is(Box::new([0]))],
        ]);
        let result = stamp.find(&query_stamp);
        let expected = vec![(0, 0)];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_be_able_to_find_positions_when_the_query_contains_any() {
        let stamp = Stamp::new(vec![
            vec![Use(1), Use(1), Use(0)],
            vec![Use(1), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let query_stamp = Stamp::new(vec![
            vec![Any, Any],
            vec![Is(Box::new([0])), Is(Box::new([0]))],
        ]);
        let result = stamp.find(&query_stamp);
        let expected = vec![(0, 1), (0, 2), (1, 2)];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_be_able_to_find_positions_using_exclusion_queries() {
        let stamp = Stamp::new(vec![
            vec![Use(1), Use(1), Use(0)],
            vec![Use(1), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let query_stamp = Stamp::new(vec![
            vec![Not(Box::new([1])), Any],
            vec![Is(Box::new([0])), Is(Box::new([0]))],
        ]);
        let result = stamp.find(&query_stamp);
        let expected = vec![(0, 2), (1, 2)];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_find_multiple_stamps() {
        let stamp = Stamp::new(vec![
            vec![Use(1), Use(1), Use(0)],
            vec![Use(1), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let query_stamp = Stamp::new(vec![vec![Is(Box::new([1]))], vec![Is(Box::new([1]))]]);
        let result = stamp.find(&query_stamp);
        let expected = vec![(0, 0), (2, 1)];
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_stamp_in_the_top_left_corner() {
        let mut stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let replace_stamp = Stamp::new(vec![vec![Use(1), Use(1)], vec![Use(1), Use(1)]]);
        stamp.stamp(&replace_stamp, 0, 0);
        let expected = Stamp::new(vec![
            vec![Use(1), Use(1), Use(0)],
            vec![Use(1), Use(1), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        assert_eq!(stamp, expected);
    }
    #[test]
    fn it_should_stamp_in_the_top_right_corner() {
        let mut stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let replace_stamp = Stamp::new(vec![vec![Use(1), Use(1)], vec![Use(1), Use(1)]]);
        stamp.stamp(&replace_stamp, 1, 0);
        let expected = Stamp::new(vec![
            vec![Use(0), Use(1), Use(1)],
            vec![Use(0), Use(1), Use(1)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        assert_eq!(stamp, expected);
    }
    #[test]
    fn it_should_stamp_in_the_bottom_left_corner() {
        let mut stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let replace_stamp = Stamp::new(vec![vec![Use(1), Use(1)], vec![Use(1), Use(1)]]);
        stamp.stamp(&replace_stamp, 0, 2);
        let expected = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(1), Use(1), Use(0)],
            vec![Use(1), Use(1), Use(0)],
        ]);
        assert_eq!(stamp, expected);
    }
    #[test]
    fn it_should_stamp_in_the_bottom_right_corner() {
        let mut stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let replace_stamp = Stamp::new(vec![vec![Use(1), Use(1)], vec![Use(1), Use(1)]]);
        stamp.stamp(&replace_stamp, 1, 2);
        let expected = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(1), Use(1)],
            vec![Use(0), Use(1), Use(1)],
        ]);
        assert_eq!(stamp, expected);
    }
    #[test]
    fn it_should_allow_transparent_stamps_to_show_through() {
        let mut stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let replace_stamp = Stamp::new(vec![vec![Transparent, Use(1)], vec![Use(1), Transparent]]);
        stamp.stamp(&replace_stamp, 1, 2);
        let expected = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(1), Use(0)],
        ]);
        assert_eq!(stamp, expected);
    }
    #[test]
    fn it_should_return_none_if_getting_coord_out_of_bound() {
        let stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let result = stamp.get_at((0, 10));
        let expected = None;
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_return_some_if_getting_coord_in_bounds() {
        let stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(2), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let result = stamp.get_at((1, 1));
        let expected = Some(&Use(2));
        assert_eq!(result, expected);
    }
    fn it_should_error_if_setting_at_coord_out_of_bounds() {}
    #[test]
    fn it_should_be_able_to_set_ok_if_coord_in_bounds() {
        let mut stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(2), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        stamp.set_at((1, 1), Use(2));
        let result = stamp.get_at((1, 1));
        let expected = Some(&Use(2));

        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_return_the_height_of_the_pattern() {
        let stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(2), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let result = stamp.height();
        let expected = 4;
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_return_the_width_of_the_pattern_if_there_are_rows() {
        let stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(2), Use(0)],
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let result = stamp.width();
        let expected = 3;
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_return_width_of_zero_if_there_are_no_rows() {
        let stamp: Stamp<StampPart<i32>> = Stamp::new(vec![]);
        let result = stamp.width();
        let expected = 0;
        assert_eq!(result, expected);
    }
    #[test]
    fn it_should_flip_horizontally() {
        let mut stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(2), Use(0)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        stamp.flip_horizontal();
        let expected = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(2), Use(0)],
            vec![Use(1), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        assert_eq!(stamp, expected);
    }
    #[test]
    fn it_should_flip_vertically() {
        let mut stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(2), Use(0)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        stamp.flip_vertical();
        let expected = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(2), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        assert_eq!(stamp, expected);
    }

    #[test]
    fn find_at_xy_should_return_true_if_stamp_is_at_xy() {
        let expected = true;
        let stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(2), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let query_stamp = Stamp::new(vec![vec![QueryStampPart::Is(Box::new([2]))]]);
        assert_eq!(stamp.find_at_xy(&query_stamp, 1, 2), expected);
    }

    #[test]
    fn find_at_xy_should_return_false_if_stamp_not_at_xy() {
        let expected = false;
        let stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(2), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let query_stamp = Stamp::new(vec![vec![QueryStampPart::Is(Box::new([2]))]]);
        assert_eq!(stamp.find_at_xy(&query_stamp, 1, 1), expected);
    }

    #[test]
    fn find_at_x_should_return_all_matching_positions_at_x() {
        let expected = vec![(1, 0), (1, 1), (1, 3)];
        let stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(2), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let query_stamp = Stamp::new(vec![vec![QueryStampPart::Is(Box::new([0]))]]);
        assert_eq!(stamp.find_at_x(&query_stamp, 1), expected);
    }

    #[test]
    fn find_at_y_should_return_all_matching_positions_at_y() {
        let expected = vec![(0, 2), (2, 2)];
        let stamp = Stamp::new(vec![
            vec![Use(0), Use(0), Use(0)],
            vec![Use(0), Use(0), Use(1)],
            vec![Use(0), Use(2), Use(0)],
            vec![Use(0), Use(0), Use(0)],
        ]);
        let query_stamp = Stamp::new(vec![vec![QueryStampPart::Is(Box::new([0]))]]);
        assert_eq!(stamp.find_at_y(&query_stamp, 2), expected);
    }
}
