use itertools::Itertools;

use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        row: usize,
        colum: usize,
        matrix: [[usize; colum]; row],
    }

    for i in 0..colum {
        let mut result: Vec<usize> = Vec::new();
        for j in 0..row {
            result.push(matrix[j][i]);
        }

        println!("{}", result.into_iter().join(" "));
    }
}
