use std::{
    collections::{BTreeSet, HashSet},
    error::Error, 
    fmt::{Display, Formatter, Result as FmtResult}
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum IntervalType {
    Start,
    End
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Point {
    row: usize,
    col: usize
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({},{})", self.row + 1, self.col + 1)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Interval {
    col: usize,
    interval_type: IntervalType,
    box_id: usize,
    top: usize,
    bottom: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    min_left_point: Point,
    max_right_point: Point,
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
    size: usize,
}

impl BoundingBox {
    fn new_with_points(points: Vec<Point>) -> Self {
        let mut min_row = points[0].row;
        let mut min_col = points[0].col;
        let mut max_row = points[0].row;
        let mut max_col = points[0].col;
        
        for point in points {
            if point.row < min_row {
                min_row = point.row;
            }
            if point.row > max_row {
                max_row = point.row;
            }
            if point.col < min_col {
                min_col = point.col;
            }
            if point.col > max_col {
                max_col = point.col;
            }
        }
        let min_left_point = Point::new(min_row, min_col);
        let max_right_point = Point::new(max_row, max_col);

        let width = max_right_point.col - min_left_point.col + 1;
        let height = max_right_point.row - min_left_point.row + 1;
        let size = width * height;

        // println!("Creating Bounding Box with left point: {:?}, right point: {:?}", min_left_point.to_string(), max_right_point.to_string());
        
        Self {
            min_left_point,
            max_right_point,
            left: min_col,
            top: max_row,
            right: max_col,
            bottom: min_row,
            size
        }
    }
}

impl Display for BoundingBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}{}", self.min_left_point, self.max_right_point)
    }
}

fn search_adjacent_points(
    points: &mut Vec<Point>, 
    row: usize, 
    col: usize, 
    matrix: &mut Vec<Vec<char>>
) {
    // check if we've visited or nothing there
    if matrix[row][col] == '-' || matrix[row][col] == 'x' {
        return;
    }

    // add position to current coordinates
    points.push(Point::new(row, col));
    matrix[row][col] = 'x';
    
    if row < (matrix.len() - 1) {
        search_adjacent_points(points, row+1, col, matrix);
    }
    if row > 0 {
        search_adjacent_points(points, row-1, col, matrix);
    }
    if col < (matrix[0].len() - 1) {
        search_adjacent_points(points, row, col+1, matrix);
    }
    if col > 0 {
        search_adjacent_points(points, row, col-1, matrix);
    }
}

fn convert_lines_to_matrix(lines: &[String]) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn validate_input(lines: &[String]) -> Result<(), String> {
    if lines.is_empty() {
        return Err(String::from("Input cannot be empty"));
    }
    let mut input_line_len = 0;

    for line in lines.iter() {
        if input_line_len == 0 {
            input_line_len = line.len();
        } else if input_line_len != line.len() {
            return Err(String::from("Lines must be the same length"));
        }
        if !line.chars().all(|ch| ch == '-' || ch == '*') {
            return Err(String::from("Characters must be either '-' or '*'"));
        }
    }
    Ok(())
}


pub fn find_boxes(lines: &[String]) -> Result<Vec<BoundingBox>, Box<dyn Error>> {
    validate_input(lines)?;
    let mut result = Vec::new();
    let mut matrix = convert_lines_to_matrix(lines);
    let mut boxes = vec![];

    // First process all boxes from matrix into BoundingBox vec
    for row in 0..matrix.len()  {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == '*' {
                let mut points = vec![];
                search_adjacent_points(&mut points, row, col, &mut matrix);
                boxes.push(BoundingBox::new_with_points(points));
            }
        }
    }

    // Find all boxes with no overlaps
    let non_overlapping_ids = get_non_overlap_boxes(&boxes);
    if non_overlapping_ids.is_empty() {
        return Ok(vec![]);
    }

    let mut largest_size = boxes[non_overlapping_ids[0]].size;
    for &id in non_overlapping_ids.iter() {
        let cur_size = boxes[id].size;
        if cur_size >= largest_size {
            largest_size = cur_size;
        }
    }
    for &id in non_overlapping_ids.iter() {
        if boxes[id].size == largest_size {
            result.push(boxes[id]);
        }
    }
    
    Ok(result)
}

fn get_non_overlap_boxes(boxes: &[BoundingBox]) -> Vec<usize> {
    // Sweep across the boxes left-right (via cols) checking intervals
    // of top/bottom for each box. Row value is greater for top vs bottom of box

    let mut result = vec![];
    let mut active: BTreeSet<(usize, usize, usize)> = BTreeSet::new();

    // create intervals
    let mut intervals = vec![];
    
    let mut overlapping: HashSet<usize> = HashSet::new();
    for (i, b) in boxes.iter().enumerate() {
        intervals.push(Interval { box_id: i, interval_type: IntervalType::Start,  col: b.left, top: b.top,  bottom: b.bottom });
        intervals.push(Interval { box_id: i, interval_type: IntervalType::End, col: b.right, top: b.top, bottom: b.bottom });
    }

    intervals.sort();

    // check each interval
    for interval in intervals.iter() {
        match interval.interval_type {
            IntervalType::Start => {
                let is_overlapping = active.iter().any(|&(bottom, top, _)| {
                    !(interval.bottom > top || interval.top < bottom)
                });
                active.insert((interval.bottom, interval.top, interval.box_id));
                if is_overlapping {
                    overlapping.insert(interval.box_id);
                    for (bottom, top, box_id) in active.iter() {
                        if !(interval.bottom > *top || interval.top < *bottom) {
                            overlapping.insert(*box_id);
                        }
                    }
                }
            },
            IntervalType::End => {
                active.remove(&(interval.bottom, interval.top, interval.box_id));
            }
        }
    }

    for (idx, _) in boxes.iter().enumerate() {
        if !overlapping.contains(&idx) {
            result.push(idx);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_lines_to_matrix1() {
        let input = vec![
            String::from("----"),
            String::from("-**-"),
            String::from("-**-"),
            String::from("----")];
        let expected1 = vec![
            vec!['-','-','-','-'],
            vec!['-','*','*','-'],
            vec!['-','*','*','-'],
            vec!['-','-','-','-'],
        ];
        assert_eq!(convert_lines_to_matrix(&input), expected1);
    }

    #[test]
    fn test_convert_lines_to_matrix2() {
        let input = vec![String::from("-")];
        let expected1 = vec![
            vec!['-']
        ];
        assert_eq!(convert_lines_to_matrix(&input), expected1);
    }

    #[test]
    fn test_search_for_adjacent_points() {
        let mut points = vec![];
        let row = 1;
        let col = 1;
        let mut matrix = vec![
            vec!['*','-','-','*'],
            vec!['-','*','*','-'],
            vec!['-','*','*','-'],
            vec!['*','-','-','-']
        ];
        search_adjacent_points(&mut points, row, col, &mut matrix);
        points.sort();
        let expected = vec![
            Point::new(1,1),
            Point::new(1,2),
            Point::new(2,1),
            Point::new(2,2),
        ];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_build_bounding_box_from_points() {
        let points1 = vec![
            Point::new(1,1),
            Point::new(1,2),
            Point::new(2,1),
            Point::new(1,1),
        ];
        let bounding_box = BoundingBox::new_with_points(points1);
        
        assert_eq!(bounding_box.min_left_point, Point::new(1,1));
        assert_eq!(bounding_box.max_right_point, Point::new(2,2));
        assert_eq!(bounding_box.left, 1);
        assert_eq!(bounding_box.top, 2);
        assert_eq!(bounding_box.right, 2);
        assert_eq!(bounding_box.bottom, 1);
        assert_eq!(bounding_box.size, 4);
    }

    #[test]
    fn test_get_non_overlap_boxes() {
        /* for example: 
         *  **-------***
         *  -*--**--***-
         *  -----***--**
         *  -------***--
         * 
         */
        let points1 = vec![
            Point::new(0,0),
            Point::new(0,1),
            Point::new(1,1),
        ];
        let bounding_box1 = BoundingBox::new_with_points(points1);
        
        let points2 = vec![
            Point::new(1,4),
            Point::new(1,5),
            Point::new(2,5),
            Point::new(2,6),
            Point::new(2,7),
            Point::new(3,7),
            Point::new(3,8),
            Point::new(3,9),
        ];
        let bounding_box2 = BoundingBox::new_with_points(points2);
        
        let points3 = vec![
            Point::new(0,9),
            Point::new(0,10),
            Point::new(0,11),
            Point::new(1,8),
            Point::new(1,9),
            Point::new(1,10),
            Point::new(2,10),
            Point::new(2,11),
        ];
        let bounding_box3 = BoundingBox::new_with_points(points3);

        let bounding_boxes = vec![bounding_box1, bounding_box2, bounding_box3];
        let non_overlapping_ids = get_non_overlap_boxes(&bounding_boxes);
        let expected = vec![0];
        assert_eq!(non_overlapping_ids, expected);
    }

    #[test]
    fn test_example_1() {
        let lines = vec![
            String::from("--*--"),
            String::from("***--"),
            String::from("----*"),
        ];

        let results = find_boxes(&lines).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].to_string(), String::from("(1,1)(2,3)"));
    }

    #[test]
    fn test_example_2() {
        let lines = vec![
            String::from("----"),
            String::from("----"),
            String::from("----"),
        ];
        let results = find_boxes(&lines).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_example_3() {
        let lines = vec![
            String::from("**--*"),
            String::from("**--*"),
            String::from("----*"),
            String::from("----*")
        ];
        let results = find_boxes(&lines).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].to_string(), String::from("(1,1)(2,2)"));
        assert_eq!(results[1].to_string(), String::from("(1,5)(4,5)"));
    }

    #[test]
    fn test_example_4() {
        let lines = vec![
            String::from("--*----"),
            String::from("-**----"),
            String::from("**-----"),
            String::from("----***"),
            String::from("----***"),
            String::from("----***")
        ];

        let results = find_boxes(&lines).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].to_string(), String::from("(1,1)(3,3)"));
        assert_eq!(results[1].to_string(), String::from("(4,5)(6,7)"));
    }

    #[test]
    fn test_example_5() {
        let lines = vec![
            String::from("-*---"),
            String::from("**-*-"),
            String::from("--**-")
        ];

        let results = find_boxes(&lines).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].to_string(), String::from("(1,1)(2,2)"));
        assert_eq!(results[1].to_string(), String::from("(2,3)(3,4)"));
    }

    #[test]
    fn test_example_6() {
        let lines = vec![
            String::from("*-*-"),
            String::from("-*-*")
        ];

        let results = find_boxes(&lines).unwrap();
        assert_eq!(results.len(), 4);
        assert_eq!(results[0].to_string(), String::from("(1,1)(1,1)"));
        assert_eq!(results[1].to_string(), String::from("(1,3)(1,3)"));
        assert_eq!(results[2].to_string(), String::from("(2,2)(2,2)"));
        assert_eq!(results[3].to_string(), String::from("(2,4)(2,4)"));
    }

    #[test]
    fn test_example_7() {
        let lines = vec![
            String::from("-*-"),
            String::from("---")
        ];
        let results = find_boxes(&lines).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].to_string(), String::from("(1,2)(1,2)"));
    }
}
