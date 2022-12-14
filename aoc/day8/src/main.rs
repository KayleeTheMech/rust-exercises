use and::and_2d;
use std::{cmp::max, fs, path::Path, vec};
pub mod and;

enum Direction {
    DOWNWARDS,
    UPWARDS,
    LEFT,
    RIGHT,
}

fn determine_x_y_width(content: &String) -> Option<(usize, usize)> {
    let width = content
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

fn viewing_distance(
    from_tree: (usize, usize),
    tree_matrix: &Vec<Vec<u8>>,
    direction: Direction,
) -> u32 {
    let initial_height = tree_matrix[from_tree.0][from_tree.1];
    viewing_distance_rec(from_tree, initial_height, tree_matrix, direction)
}

fn viewing_distance_rec(
    from_tree: (usize, usize),
    initial_height: u8,
    tree_matrix: &Vec<Vec<u8>>,
    direction: Direction,
) -> u32 {
    let mut cursor = from_tree;
    if cursor_at_edge(cursor, tree_matrix) {
        return 0;
    }
    let next_cursor = match direction {
        Direction::DOWNWARDS => (cursor.0 + 1, cursor.1),
        Direction::UPWARDS => (cursor.0 - 1, cursor.1),
        Direction::LEFT => (cursor.0, cursor.1 - 1),
        Direction::RIGHT => (cursor.0, cursor.1 + 1),
    };
    if initial_height > tree_matrix[next_cursor.0][next_cursor.1] {
        return viewing_distance_rec(next_cursor, initial_height, tree_matrix, direction) + 1;
    } else {
        return 1;
    }
}

fn scenic_factor(from_tree: (usize, usize), tree_matrix: &Vec<Vec<u8>>) -> u32 {
    let view_down = viewing_distance(
        (from_tree.0, from_tree.1),
        &tree_matrix,
        Direction::DOWNWARDS,
    );
    let view_up = viewing_distance((from_tree.0, from_tree.1), &tree_matrix, Direction::UPWARDS);
    let view_left = viewing_distance((from_tree.0, from_tree.1), &tree_matrix, Direction::LEFT);
    let view_right = viewing_distance((from_tree.0, from_tree.1), &tree_matrix, Direction::RIGHT);
    view_down * view_up * view_left * view_right
}

#[test]
fn test_get_trees() {
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
}

#[test]
fn test_viewing_distance() {
    let reference: Vec<Vec<u8>> = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];
    assert_eq!(1, viewing_distance((1, 2), &reference, Direction::UPWARDS));
    assert_eq!(
        2,
        viewing_distance((1, 2), &reference, Direction::DOWNWARDS)
    );
    assert_eq!(1, viewing_distance((1, 2), &reference, Direction::LEFT));
    assert_eq!(2, viewing_distance((1, 2), &reference, Direction::RIGHT));
    assert_eq!(2, viewing_distance((3, 2), &reference, Direction::LEFT));
    assert_eq!(2, viewing_distance((3, 2), &reference, Direction::RIGHT));
    let reference: Vec<Vec<u8>> = vec![
        vec![3, 0, 3, 7, 3, 1, 1, 1],
        vec![2, 5, 5, 1, 2, 1, 1, 1],
        vec![6, 5, 3, 3, 2, 1, 1, 1],
        vec![3, 3, 5, 4, 3, 1, 4, 9],
        vec![3, 5, 3, 9, 0, 1, 1, 1],
    ];
    assert_eq!(5, viewing_distance((3, 2), &reference, Direction::RIGHT));
}

#[test]
fn test_find_hidden_ones() {
    let input = "30373\n25512\n65332\n33549\n35390\n".to_string();
    let trees = get_tree_matrix(&input).unwrap();
    let reference: Vec<Vec<bool>> = vec![
        vec![false, false, false, false, false],
        vec![false, false, true, true, false],
        vec![false, true, true, true, false],
        vec![false, true, false, true, false],
        vec![false, false, false, false, false],
    ];

    assert_eq!(
        get_hidden_matrix_in_direction(&trees, Direction::RIGHT),
        reference
    )
}

#[test]
fn test_scenic_factor() {
    let reference: Vec<Vec<u8>> = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];
    assert_eq!(4, scenic_factor((1, 2), &reference));
    assert_eq!(8, scenic_factor((3, 2), &reference));
}

fn is_tree_hidden(location: (usize, usize), hidden_tree_matrix: &Vec<Vec<bool>>) -> bool {
    hidden_tree_matrix[location.0][location.1]
}

fn main() {
    let filepath = Path::new("./input.txt");
    let content = fs::read_to_string(filepath).expect("Couldn't read input.txt");
    let trees = get_tree_matrix(&content).expect("Expecting to be able to read all trees");
    println!("Scanning trees done!");
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
    for line in hidden_tree_matrix.as_slice() {
        for tree_hidden in line.as_slice() {
            if !tree_hidden {
                trees_visible += 1;
            }
        }
    }
    println!("{:?} trees are visible.", trees_visible);

    let mut max_scenic_factor: u32 = 0;
    for line in 0..trees.len() {
        for column in 0..trees[0].len() {
            let scenic_factor = scenic_factor((line, column), &trees);
            max_scenic_factor = max(max_scenic_factor, scenic_factor);
        }
    }
    println!("Max Scenic factor {:?}", max_scenic_factor);
}
