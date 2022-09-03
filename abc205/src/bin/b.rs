use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    a.sort();
    for (i, v) in a.iter().enumerate() {
        if i + 1 != *v {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
