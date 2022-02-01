use num::{pow, PrimInt};
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: i64
    }

    if n < 2.pow(31) && -(2.pow(31)) <= n {
        print!("Yes")
    } else {
        print!("No")
    }
}
