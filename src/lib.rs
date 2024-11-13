use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize
}
struct BoundingBox {
    coordinates: HashSet<Point>,
    min_left_point: Option<Point>,
    min_right_point: Option<Point>,
    max_left_point: Option<Point>,
    max_right_point: Option<Point>,
}

impl BoundingBox {
    fn new() -> Self {
        Self {
            coordinates: HashSet::new(),
            min_left_point: None,
            min_right_point: None,
            max_left_point: None,
            max_right_point: None,
        }
    }

    fn print_all_bounds(&self) {
        let min_left = self.min_left_point.as_ref().unwrap();
        let min_right = self.min_right_point.as_ref().unwrap();
        let max_left = self.max_left_point.as_ref().unwrap();
        let max_right = self.max_right_point.as_ref().unwrap();
        println!("\n({:?},{:?})({:?},{:?})", min_left.x, min_left.y, min_right.x, min_right.y);
        println!("({:?},{:?})({:?},{:?})", max_left.x, max_left.y, max_right.x, max_right.y)
    }
}

fn search_island(
    bounding_box: &mut BoundingBox, 
    row: usize, 
    col: usize, 
    matrix: &mut Vec<Vec<char>>
) {
    // check if we've visited or nothing there
    if matrix[row][col] == '-' || matrix[row][col] == 'x' {
        return;
    }
    // add position to current coordinates
    bounding_box.coordinates.insert(Point { x: row, y: col });
    
    match &mut bounding_box.min_left_point {
        Some(min_left_point) => {
            if col < min_left_point.y {
                min_left_point.y = col;
            }
            if row < min_left_point.x {
                min_left_point.x = row;
            }
        },
        None => bounding_box.min_left_point = Some(Point { x: row, y: col}),
    }

    match &mut bounding_box.max_right_point {
        Some(max_right_point) => {
            if col > max_right_point.y {
                max_right_point.y = col;
            }
            if row > max_right_point.x {
                max_right_point.x = row;
            }
        },
        None => bounding_box.max_right_point = Some(Point { x: row, y: col}),
    }
    
    matrix[row][col] = 'x';
    
    if row < (matrix.len() - 1) {
        search_island(bounding_box, row+1, col, matrix);
    }
    if row > 0 {
        search_island(bounding_box, row-1, col, matrix);
    }
    if col < (matrix[0].len() - 1) {
        search_island(bounding_box, row, col+1, matrix);
    }
    if col > 0 {
        search_island(bounding_box, row, col-1, matrix);
    }
}

pub fn min_bounding_boxes(data: &str) -> String {
    // Convert to 2d array
    let mut matrix: Vec<Vec<char>> = data
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut boxes = vec![];
    for row in 0..matrix.len()  {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == '*' {
                let mut bounding_box = BoundingBox::new();
                search_island(&mut bounding_box, row, col, &mut matrix);
                if bounding_box.min_left_point.is_some() && bounding_box.max_right_point.is_some() {
                    let top_left = bounding_box.min_left_point.as_ref().unwrap();
                    let bottom_right = bounding_box.max_right_point.as_ref().unwrap();
                    bounding_box.min_right_point = Some(Point { x: top_left.x, y: bottom_right.y });
                    bounding_box.max_left_point = Some(Point { x: bottom_right.x, y: top_left.y });
                }
                // bounding_box.min_right_point = Some(Point { x: , y: })
                boxes.push(bounding_box);
            }
        }
    }

    for b in boxes.iter() {
        b.print_all_bounds();
    }


    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_input() {
        let input ="\
----
-**-
-**-
----
";
        assert_eq!(min_bounding_boxes(&input), "(2,2)(3,3)");
    }
}