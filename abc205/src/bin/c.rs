use proconio::{fastout, input};
use std::cmp::Ordering::*;

#[fastout]
fn main() {
    input! {
        (mut a, mut b, c): (i32, i32, u32),
    };
    if c % 2 == 0 {
        a = a.abs();
        b = b.abs();
    }
    let ans = match a.cmp(&b) {
        Greater => ">",
        Less => "<",
        Equal => "=",
    };
    println!("{}", ans);
}
