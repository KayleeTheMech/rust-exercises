use std::{fs, path::Path, vec};

enum Direction {
    DOWNWARDS,
    UPWARDS,
    LEFT,
    RIGHT,
}

fn determine_x_y_width(content: &String) -> Option<(usize, usize)> {
    let mut width = content
        .trim_end_matches("\n")
        .split("\n")
        .next()?
        .chars()
        .count();
    let height = content.trim_end_matches("\n").split("\n").count();

    Some((width, height))
}

fn get_tree_matrix(content: &String) -> Option<Vec<Vec<u8>>> {
    let (width, height) = determine_x_y_width(&content).unwrap();
    let mut trees = vec![vec![0 as u8; width]; height];
    let mut cursor: (usize, usize) = (0, 0);

    for line in content.trim_end_matches("\n").split("\n") {
        cursor = (cursor.0, 0); // reset column to zero
        for tree in line.chars() {
            trees[cursor.0][cursor.1] = tree.to_digit(10)? as u8;
            cursor = (cursor.0, cursor.1 + 1) // set cursor to next tree
        }
        cursor = (cursor.0 + 1, cursor.1) // increase line counter at end of loop
    }
    Some(trees)
}

fn cursor_at_edge<T>(cursor: (usize, usize), matrix: &Vec<Vec<T>>) -> bool {
    let max_width = matrix[0].len();
    let max_height = matrix.len();
    let (line, column) = cursor;
    if line == 0 {
        true
    } else if line == max_height - 1 {
        true
    } else if column == 0 {
        true
    } else if column == max_width - 1 {
        true
    } else {
        false
    }
}

fn get_hidden_matrix_in_direction(
    tree_matrix: &Vec<Vec<u8>>,
    direction: Direction,
) -> Vec<Vec<bool>> {
    let mut hidden_matrix = vec![vec![true; tree_matrix[0].len()]; tree_matrix.len()];
    let lines = tree_matrix.len();
    let columns = tree_matrix[0].len();
    let mut cursor: (usize, usize) = match direction {
        // (line, column) of starting cursor
        Direction::DOWNWARDS => (0, 0),       // upper left corner
        Direction::UPWARDS => (lines - 1, 0), // lower left corner
        Direction::RIGHT => (0, 0),           // upper left corner
        Direction::LEFT => (0, columns - 1),  // upper right corner
    };
    // move over lines or columns
    let mut max_height: u8 = 0;
    loop {
        // do work
        if cursor_at_edge(cursor, tree_matrix) && max_height == 0 {
            // tree is visible
            hidden_matrix[cursor.0][cursor.1] = false;
            max_height = tree_matrix[cursor.0][cursor.1];
        } else if cursor_at_edge(cursor, tree_matrix) {
            hidden_matrix[cursor.0][cursor.1] = false;
        } else if tree_matrix[cursor.0][cursor.1] > max_height {
            // tree is visible
            hidden_matrix[cursor.0][cursor.1] = false;
            max_height = tree_matrix[cursor.0][cursor.1];
        }

        // iterator and break condition
        match direction {
            Direction::DOWNWARDS => {
                // loop end condition
                if cursor == (lines - 1, columns - 1) {
                    // ends in lower right corner
                    break;
                } else {
                    // move cursor
                    if cursor.0 == lines - 1 {
                        // reset line, move to next column
                        cursor = (0, cursor.1 + 1);
                        max_height = 0;
                    } else if max_height == 9 {
                        // rest is hidden anyway
                        // reset line, move to next column
                        cursor = (0, cursor.1 + 1);
                        max_height = 0;
                    } else {
                        // move to next line downwards
                        cursor = (cursor.0 + 1, cursor.1);
                    }
                }
            }
            Direction::UPWARDS => {
                // loop end condition
                if cursor == (0, columns - 1) {
                    // ends in upper right corner
                    break;
                } else {
                    // move cursor
                    if cursor.0 == 0 {
                        // reset line, move to next column
                        cursor = (lines - 1, cursor.1 + 1);
                        max_height = 0;
                    } else if max_height == 9 {
                        // rest is hidden anyway
                        // reset line, move to next column
                        cursor = (lines - 1, cursor.1 + 1);
                        max_height = 0;
                    } else {
                        // move to next line upwards
                        cursor = (cursor.0 - 1, cursor.1);
                    }
                }
            }
            Direction::RIGHT => {
                // loop end condition
                if cursor == (lines - 1, columns - 1) {
                    // ends in lower right corner
                    break;
                } else {
                    // move cursor
                    if cursor.1 == columns - 1 {
                        // reset column, move to next line
                        cursor = (cursor.0 + 1, 0);
                        max_height = 0;
                    } else if max_height == 9 {
                        // reset column, move to next line
                        cursor = (cursor.0 + 1, 0);
                        max_height = 0;
                    } else {
                        // move to next left column
                        cursor = (cursor.0, cursor.1 + 1);
                    }
                }
            }
            Direction::LEFT =>
            // loop end condition
            {
                if cursor == (lines - 1, 0) {
                    // ends in lower left corner
                    break;
                } else {
                    // move cursor
                    if cursor.1 == 0 {
                        // reset column, move to next line
                        cursor = (cursor.0 + 1, columns - 1);
                        max_height = 0;
                    } else if max_height == 9 {
                        // reset column, move to next line
                        cursor = (cursor.0 + 1, columns - 1);
                        max_height = 0;
                    } else {
                        // move to next right column
                        cursor = (cursor.0, cursor.1 - 1);
                    }
                }
            }
        }
    }
    hidden_matrix
}

// vectors of equal size
fn and_1d(vector1: &Vec<bool>, vector2: &Vec<bool>) -> Vec<bool> {
    let mut result = vec![false; vector1.len()];
    for ((resref, val1), val2) in result.iter_mut().zip(vector1).zip(vector2) {
        *resref = val1 & val2;
    }
    result
}

#[test]
fn test_and_1d() {
    let vec1 = vec![true, false, false];
    let vec2 = vec![true, false, true];
    let result = and_1d(&vec1, &vec2);
    assert_eq!(result[0], true);
    assert_eq!(result[1], false);
    assert_eq!(result[2], false);
}

// matrix of equal dimension
fn and_2d(matrix1: &Vec<Vec<bool>>, matrix2: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut result = vec![vec![false; matrix1[0].len()]; matrix1.len()];
    for ((resref, vec1), vec2) in result.iter_mut().zip(matrix1).zip(matrix2) {
        *resref = and_1d(vec1, vec2)
    }
    result
}

#[test]
fn test_and_2d() {
    let mat1 = vec![vec![true, false], vec![false, true]];
    let mat2 = vec![vec![true, true], vec![true, true]];
    assert_eq!(
        and_2d(&mat1, &mat2),
        vec![vec![true, false], vec![false, true]]
    );
    let mat3 = vec![vec![false, false], vec![false, false]];
    assert_eq!(
        and_2d(&mat1, &mat3),
        vec![vec![false, false], vec![false, false]]
    );
}

#[test]
fn test_get_trees_and_find_hidden_ones() {
    let input = "30373\n25512\n65332\n33549\n35390\n".to_string();
    let trees = get_tree_matrix(&input).unwrap();
    let reference: Vec<Vec<u8>> = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];
    assert_eq!(trees, reference);

    let reference: Vec<Vec<bool>> = vec![
        vec![false, true, true, false, true],
        vec![false, false, true, true, true],
        vec![false, true, true, true, true],
        vec![false, true, false, true, false],
        vec![false, false, true, false, true],
    ];
    assert_eq!(
        get_hidden_matrix_in_direction(&trees, Direction::RIGHT),
        reference
    )
}

fn main() {
    let filepath = Path::new("./input.txt");
    let content = fs::read_to_string(filepath).expect("Couldn't read input.txt");
    let trees = get_tree_matrix(&content).expect("Expecting to be able to read all trees");
    println!("Scanning trees done!");
    let (width, height) = determine_x_y_width(&content)
        .expect("Expecting to get a read of the size of the tree field");

    println!("Looking from all directions");
    let matrix_left = get_hidden_matrix_in_direction(&trees, Direction::LEFT);
    let matrix_right = get_hidden_matrix_in_direction(&trees, Direction::RIGHT);
    let matrix_up = get_hidden_matrix_in_direction(&trees, Direction::UPWARDS);
    let matrix_downwards = get_hidden_matrix_in_direction(&trees, Direction::DOWNWARDS);
    println!("Find the trees that are hidden from all directions");
    let left_right = and_2d(&matrix_left, &matrix_right);
    let up_down = and_2d(&matrix_up, &matrix_downwards);
    let hidden_tree_matrix = and_2d(&left_right, &up_down);

    let mut trees_visible: u32 = 0;
    for line in hidden_tree_matrix {
        for tree_hidden in line {
            if !tree_hidden {
                trees_visible += 1;
            }
        }
    }
    println!("{:?} trees are visible.", trees_visible);
}
