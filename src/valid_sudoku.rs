use std::{collections::HashMap, iter::Map};

pub fn test1() {
    let input1: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    let input2: Vec<Vec<char>> = vec![
        vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    let input3: Vec<Vec<char>> = vec![
        vec!['7', '.', '.', '.', '4', '.', '.', '.', '.'],
        vec!['.', '.', '.', '8', '6', '5', '.', '.', '.'],
        vec!['.', '1', '.', '2', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '9', '.', '.', '.'],
        vec!['.', '.', '.', '.', '5', '.', '5', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '2', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ];

    let input4: Vec<Vec<char>> = vec![
        vec!['.', '.', '.', '9', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '3', '.', '.', '.', '.', '.', '1'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['1', '.', '.', '.', '.', '.', '3', '.', '.'],
        vec!['.', '.', '.', '.', '2', '.', '6', '.', '.'],
        vec!['.', '9', '.', '.', '.', '.', '.', '7', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['8', '.', '.', '8', '.', '.', '.', '.', '.'],
    ];

    assert!(is_valid_sudoku(input1));
    assert!(!is_valid_sudoku(input2));
    assert!(!is_valid_sudoku(input3));
    assert!(!is_valid_sudoku(input4));
}

pub fn test_sandbox() {
    let input1: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    // just sandbox part
    // iter_sandbox();
    let col1: Vec<&char> = pick_col(&input1, 1).collect();
    println!("col1: {col1:?}");
    let row3: Vec<_> = pick_row(&input1, 2).collect();
    println!("row3: {row3:?}");
    let square1: Vec<&char> = pick_square(&input1, 0, 0).collect();
    println!("square1: {square1:?}");

    let square2: Vec<&char> = pick_square(&input1, 1, 1).collect();
    println!("square1: {square2:?}");

    let strange = vec!['.', '.', '.', '.', '5', '.', '5', '.', '.'];
    let strange_res = no_repeats(strange.iter());
    println!("strange_res: {strange_res}");
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    if !ensure_size(&board) {
        return false;
    }
    for i in 0..9 {
        let row_i = pick_row(&board, i);
        if !no_repeats(row_i) {
            // println!("bad row: {i:?}");
            return false;
        }
        let col_i = pick_col(&board, i);
        if !no_repeats(col_i) {
            // println!("bad col: {i:?}");
            return false;
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            let square = pick_square(&board, i, j);
            if !no_repeats(square) {
                println!("bad square: {i:?}, {j:?}");
                return false;
            }
        }
    }
    return true;
}

fn ensure_size(board: &[Vec<char>]) -> bool {
    let rows_checked = board.len() == 9;
    let mut cols_checked = true;
    for row in board {
        if row.len() != 9 {
            cols_checked = false;
            break;
        }
    }
    rows_checked && cols_checked
}

fn pick_row(board: &[Vec<char>], index: usize) -> impl Iterator<Item = &char> {
    let out = board.get(index).expect("Wrong row index").iter();
    out
}

fn pick_col(board: &[Vec<char>], index: usize) -> impl Iterator<Item = &char> {
    let out = board.iter().filter_map(move |row| row.get(index));
    out
}

fn pick_square(board: &[Vec<char>], row: usize, col: usize) -> impl Iterator<Item = &char> {
    let board_p = board
        .iter()
        .skip(row * 3)
        .take(3)
        .flat_map(move |row_elem| row_elem.iter().skip(col * 3).take(3));
    board_p
}

fn no_repeats<'a>(numbers: impl Iterator<Item = &'a char>) -> bool {
    let mut repeat_map: HashMap<char, usize> = HashMap::new();
    for number in numbers {
        if *number == '.' {
            continue;
        }
        let repeated = repeat_map.insert(*number, 0);
        match repeated {
            Some(_) => {
                return false;
            }
            None => {
                continue;
            }
        }
    }
    return true;
}

fn iter_sandbox() {
    let example_slice = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5];
    let e_iter_f = example_slice.iter().filter(|elem| (*elem % 2) == 0);
    println!("e_iter value: {:?}", e_iter_f.collect::<Vec<_>>());
    println!("example_slice: {example_slice:?}"); // iterator doesn't takes ownership
    let e_iter_fm = example_slice
        .iter()
        .filter_map(|&elem| if elem % 2 == 0 { Some(elem * 10) } else { None });
    println!("e_iter_fm: {:?}", e_iter_fm.collect::<Vec<_>>());
}
