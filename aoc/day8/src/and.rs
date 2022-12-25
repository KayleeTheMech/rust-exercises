// vectors of equal size
pub fn and_1d(vector1: &Vec<bool>, vector2: &Vec<bool>) -> Vec<bool> {
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
pub fn and_2d(matrix1: &Vec<Vec<bool>>, matrix2: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
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
